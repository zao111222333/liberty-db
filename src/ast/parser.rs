use nom::{
  error::{Error, ContextError, FromExternalError, ParseError, ErrorKind, VerboseError}, 
  IResult, 
  combinator::{map, opt, map_res, map_opt},
  sequence::{tuple, delimited, preceded, pair, terminated},
  character::streaming::{char, none_of, one_of},
  branch::alt,
  bytes::{streaming::{take_while, escaped, take_until, take}, complete::tag}, 
  multi::{separated_list0, many0}, 
  InputTakeAtPosition, 
};
fn comment_single<'a>(
  i: &'a str,
) -> IResult<&'a str, usize, Error<&'a str>> 
{
  map(
    tuple((
      alt((
        tag("*"),
        tag("//"),
      )),
      take_until("\n"),
      take(1usize),
      space,
    )),
    |_|1
  )(i)
}

fn comment_multi<'a>(
  i: &'a str,
) -> IResult<&'a str, usize, Error<&'a str>> 
{
  map(
    tuple((
      tag("/*"),
      take_until("*/"),
      tag("*/"),
      space,
    )),
    |(_,s,_,_)|s.chars().filter(|&x| x == '\n').count()
  )(i)
}


#[test]
fn comment_test(){
  println!("{:?}",comment_single("iwww\" \n \nw"));
  println!("{:?}",comment_single("*iwww\" \n \nw"));
  println!("{:?}",comment_single("//iwww\" \n \nw"));
  println!("{:?}",comment_multi(r#"/*iwww\

  */
  w"#));
  println!("{:?}",comment_multi(r#"/*iwww\ */
  w"#));
}

#[inline]
pub fn space<'a>(i: &'a str) -> IResult<&'a str, (), Error<&'a str>> 
{
  map(
    take_while(move |c| " \t\r".contains(c)), 
    |_|(),
  )(i)
}
#[inline]
pub fn space_newline<'a>(i: &'a str) -> IResult<&'a str, usize, Error<&'a str>> 
{
  map(
    take_while(move |c| " \t\r\n".contains(c)), 
    |s: &str|s.chars().filter(|&x| x == '\n').count(),
  )(i)
}

#[inline]
pub fn space_newline_slash<'a>(i: &'a str) -> IResult<&'a str, usize, Error<&'a str>> 
{
  map(
    pair(
      space, 
      opt(
          preceded(
            preceded(
              char('\\'),
              space,
            ),
            alt((
              map_opt(
                pair(
                  opt(comment_multi),
                  space_newline,
                ),
                |(n_comment,n_newline)| match (n_comment,n_newline) {
                  (_,0) => None,
                  (Some(0),_) => Some(n_newline),
                  (None,_) => Some(n_newline),
                  (_,_) => None,
                },
              ),
              comment_single,
            )),
          )
        ),
      ),
    |(_,n)| match n {
        Some(n) => n,
        None => 0,
    }
  )(i)
}

#[test]
fn space_test(){
  println!("{:?}",space_newline_slash(r#"/*iwww\
  
  */
  w"#));
  println!("{:?}",space_newline_slash(r#"\ /*iwww\*/
  w"#));
  println!("{:?}",space_newline_slash(r#"\
  w"#));
  println!("{:?}",space_newline_slash(r#"\ //www
  w"#));
}

#[inline]
pub fn undefine<'a>(i: &'a str) -> IResult<&'a str, super::AttriValue, Error<&'a str>> 
{
  todo!()
}

#[inline]
fn unquote<'a, E>(
  i: &'a str,
) -> IResult<&'a str, &'a str, E> 
where
  E: ParseError<&'a str> 
    + ContextError<&'a str> 
    + FromExternalError<&'a str, E>
{
  delimited(
    char('"'),
    escaped(
      opt(none_of(r#"\""#)), 
      '\\', 
      one_of(r#"\"rnt"#),
    ),
    char('"'),
  )
  (i)
}

#[test]
fn unquote_test(){
  println!("{:?}",unquote::<VerboseError<&str>>("\"iwww\" "));
  println!("{:?}",key::<VerboseError<&str>>("iw_ww "));
  println!("{:?}",key::<VerboseError<&str>>("iw_w2w "));
  println!("{:?}",key::<VerboseError<&str>>("iw_w2w';"));
}


pub fn key<'a, E>(
  i: &'a str,
) -> IResult<&'a str, &'a str, E> 
where
  E: ParseError<&'a str> 
    + ContextError<&'a str> 
    + FromExternalError<&'a str, E>
{
  i.split_at_position1(
    |item| !(item.is_alphanumeric()||item=='_'), 
    ErrorKind::Alpha
  )
}

pub fn word<'a, E>(
  i: &'a str,
) -> IResult<&'a str, &'a str, E> 
where
  E: ParseError<&'a str> 
    + ContextError<&'a str> 
    + FromExternalError<&'a str, E>
{
  i.split_at_position1(|item| 
    !(item.is_alphanumeric()||"/_.+-".contains(item)), 
    ErrorKind::Alpha
  )
}

pub(crate) fn simple<'a>(
  i: &'a str, line_num: &mut usize,
) -> IResult<&'a str, &'a str, Error<&'a str>>
{
  map(
    tuple((
      space,
      char(':'),
      space,
      alt((
        unquote,
        word,
      )),
      space,
      char(';'),
      space_newline,
    )),
    |(_,_,_,s,_,_,n)| {
        *line_num += n;
        s
      }
  )(i)
}


fn complex_complex<'a>(
  i: &'a str,
) -> IResult<&'a str, Vec<&'a str>, Error<&'a str>>
{
  let (input, words) = unquote(i)?;
  Ok((input, words.split(',').filter_map(|s|{
    let _s = s.trim(); 
    if _s==""{None}else{Some(_s)}}
  ).collect()))
}
pub(crate) fn complex<'a>(
  i: &'a str, line_num: &mut usize,
) -> IResult<&'a str, Vec<Vec<&'a str>>, Error<&'a str>>
{
  map(
    tuple((
      space,
      char('('),
      space_newline_slash,
      many0(
        pair(
          alt((
            map(word, |s|vec![s]),
            complex_complex,
          )),
          preceded(
            char(','),
            space_newline_slash,
          ),
        )
      ),
      opt(
        pair(
          alt((
            complex_complex,
            map(word, |s| vec![s]),
          )),
          space_newline_slash,
        ),
      ),
      char(')'),
      space,
      char(';'),
      space_newline,
    )),
    |(_,_,n0,res,last,_,_,_,n1)| {
      *line_num += n0+n1;
      let mut vec = res.into_iter().map(|(v,n)|{
        *line_num += n;
        v
      }).collect::<Vec<Vec<&'a str>>>();
      // }).flatten().collect::<Vec<&'a str>>();
      if let Some((mut last_vec,n)) = last{
        *line_num += n;
        // vec.append(&mut last_vec);
        vec.push(last_vec)
      }
      vec
    },
  )(i)
}

#[test]
fn key_test(){
  println!("{:?}",space_newline("\n\r\t\n : b ; "));
  println!("{:?}",simple(" : b; }", &mut 1));
  println!("{:?}",simple(" : iwww ; ", &mut 1));
  println!("{:?}",key::<VerboseError<&str>>("iwww "));
  println!("{:?}",key::<VerboseError<&str>>("iw_ww "));
  println!("{:?}",key::<VerboseError<&str>>("iw_w2w "));
  println!("{:?}",key::<VerboseError<&str>>("iw_w2w';"));
}


pub fn title<'a>(
  i: &'a str, line_num: &mut usize,
) -> IResult<&'a str, Vec<&'a str>, Error<&'a str>>
{
  map(
    tuple((
      space,
      char('('),
      separated_list0(
        preceded(
          space,
          char(','),
        ),
        alt((
          unquote,
          word,
        )),
      ),
      char(')'),
      space,
      char('{'),
      space_newline,
    )),
    |(_,_,x,_,_,_,n)| {
      *line_num += n;
      x
    },
  )(i)
}

pub fn end_group<'a>(
  i: &'a str, 
) -> IResult<&'a str, (), Error<&'a str>>
{
  map(
    delimited(
      char('}'), 
      space,
      opt(char(';')),
    ),
    |_|())(i)
}