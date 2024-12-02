mod demo {
    #![allow(clippy::redundant_pub_crate, clippy::doc_markdown)]
    //! cargo expand common::demo
    //! cargo expand common::demo --no-default-features
    use crate::{
        ast::{Attributes, GroupComments, GroupFn, GroupSet, NamedGroup},
        cell::Statetable, timing::TimingType, ArcStr, NotNan,
    };
    use core::fmt::Write;
    pub(crate) struct Timing {
        /// group undefined attributes
        #[liberty(attributes)]
        attributes: Attributes,
        /// group comments
        #[liberty(comments)]
        comments: GroupComments,
        #[liberty(complex)]
        #[default = "vec![unsafe{ NotNan::new_unchecked(0.0) }]"]
        pub values: Vec<NotNan<f64>>,
        #[liberty(simple(type = Option))]
        t1: Option<TimingType>,
        #[liberty(simple(type = Option))]
        t2: Option<TimingType>,
    }
    #[doc(hidden)]
    impl Default for Timing {
        #[inline]
        fn default() -> Self {
            Self {
                attributes: Default::default(),
                comments: Default::default(),
                values: <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([unsafe { NotNan::new_unchecked(0.0) }]),
                ),
                t1: Default::default(),
                t2: Default::default(),
            }
        }
    }
    impl Timing {
        #[inline]
        pub fn comments_this(&self) -> Option<&String> {
            self.comments.0.get(&187108777618759007u64)
        }
        #[inline]
        pub fn comments_this_entry<'a>(
            &'a mut self,
        ) -> std::collections::hash_map::Entry<'a, u64, String> {
            self.comments.0.entry(187108777618759007u64)
        }
        #[inline]
        pub fn comments_values(&self) -> Option<&String> {
            self.comments.0.get(&23965487694171063u64)
        }
        #[inline]
        pub fn comments_values_entry<'a>(
            &'a mut self,
        ) -> std::collections::hash_map::Entry<'a, u64, String> {
            self.comments.0.entry(23965487694171063u64)
        }
        #[inline]
        pub fn comments_t1(&self) -> Option<&String> {
            self.comments.0.get(&36849809221889801u64)
        }
        #[inline]
        pub fn comments_t1_entry<'a>(
            &'a mut self,
        ) -> std::collections::hash_map::Entry<'a, u64, String> {
            self.comments.0.entry(36849809221889801u64)
        }
        #[inline]
        pub fn comments_t2(&self) -> Option<&String> {
            self.comments.0.get(&14976594826780549248u64)
        }
        #[inline]
        pub fn comments_t2_entry<'a>(
            &'a mut self,
        ) -> std::collections::hash_map::Entry<'a, u64, String> {
            self.comments.0.entry(14976594826780549248u64)
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::too_many_lines
    )]
    impl crate::ast::Group for Timing {}
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::too_many_lines
    )]
    impl crate::ast::GroupAttri for Timing {
        fn fmt_liberty<T: core::fmt::Write, I: crate::ast::Indentation>(
            &self,
            key: &str,
            f: &mut crate::ast::CodeFormatter<'_, T, I>,
        ) -> core::fmt::Result {
            use core::fmt::Write;
            use itertools::Itertools;
            let indent = f.indentation();
            f.write_fmt(format_args!("\n{0}{1} () {{", indent, key))?;
            f.indent(1);
            crate::ast::fmt_comment_liberty(self.comments_values(), f)?;
            crate::ast::ComplexAttri::fmt_liberty(&self.values, "values", f)?;
            if let Some(simple) = &self.t1 {
                crate::ast::fmt_comment_liberty(self.comments_t1(), f)?;
                crate::ast::SimpleAttri::fmt_liberty(simple, "t1", f)?;
            }
            if let Some(simple) = &self.t2 {
                crate::ast::fmt_comment_liberty(self.comments_t2(), f)?;
                crate::ast::SimpleAttri::fmt_liberty(simple, "t2", f)?;
            }
            if !self.attributes.is_empty() {
                crate::ast::attributs_fmt_liberty(&self.attributes, f)?;
            }
            f.dedent(1);
            f.write_fmt(format_args!("\n{0}}}", indent))
        }
        fn nom_parse<'a>(
            i: &'a str,
            group_name: &str,
            scope: &mut crate::ast::ParseScope,
        ) -> nom::IResult<
            &'a str,
            Result<Self, crate::ast::IdError>,
            nom::error::Error<&'a str>,
        > {
            let (mut input, title) = crate::ast::parser::title(i, &mut scope.line_num)?;
            let mut attributes = Default::default();
            let mut values = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([unsafe { NotNan::new_unchecked(0.0) }]),
            );
            let mut t1 = Default::default();
            let mut t2 = Default::default();
            loop {
                match crate::ast::parser::key(input) {
                    Err(nom::Err::Error(_)) => {
                        (input, _) = crate::ast::parser::end_group(input)?;
                        let mut res = Self {
                            attributes,
                            values,
                            t1,
                            t2,
                            ..Default::default()
                        };
                        <Self as crate::ast::GroupFn>::post_parse_process(
                            &mut res,
                            scope,
                        );
                        return Ok((input, Ok(res)));
                    }
                    Err(e) => return Err(e),
                    Ok((_input, key)) => {
                        input = _input;
                        #[deny(unreachable_patterns)]
                        match key {
                            "values" => {
                                let (new_input, complex_res) = crate::ast::ComplexAttri::nom_parse(
                                    input,
                                    scope,
                                )?;
                                input = new_input;
                                match complex_res {
                                    Ok(complex) => values = complex,
                                    Err((e, undefined)) => {
                                        {
                                            let lvl = ::log::Level::Error;
                                            if lvl <= ::log::STATIC_MAX_LEVEL
                                                && lvl <= ::log::max_level()
                                            {
                                                ::log::__private_api::log(
                                                    format_args!(
                                                        "Line={0}; Key={1}; Value={2:?}; Err={3}",
                                                        scope.line_num,
                                                        key,
                                                        undefined,
                                                        e,
                                                    ),
                                                    lvl,
                                                    &(
                                                        "liberty_db::common::demo",
                                                        "liberty_db::common::demo",
                                                        ::log::__private_api::loc(),
                                                    ),
                                                    (),
                                                );
                                            }
                                        };
                                        crate::ast::attributs_set_undefined_complex(
                                            &mut attributes,
                                            key,
                                            undefined,
                                        );
                                    }
                                }
                            }
                            "t1" => {
                                let (new_input, simple_res) = crate::ast::SimpleAttri::nom_parse(
                                    input,
                                    scope,
                                )?;
                                input = new_input;
                                match simple_res {
                                    Ok(simple) => {
                                        t1 = Some(simple);
                                    }
                                    Err(undefined) => {
                                        {
                                            let lvl = ::log::Level::Error;
                                            if lvl <= ::log::STATIC_MAX_LEVEL
                                                && lvl <= ::log::max_level()
                                            {
                                                ::log::__private_api::log(
                                                    format_args!(
                                                        "Line={0}; Key={1}; Value={2:?}",
                                                        scope.line_num,
                                                        key,
                                                        undefined,
                                                    ),
                                                    lvl,
                                                    &(
                                                        "liberty_db::common::demo",
                                                        "liberty_db::common::demo",
                                                        ::log::__private_api::loc(),
                                                    ),
                                                    (),
                                                );
                                            }
                                        };
                                        crate::ast::attributs_set_undefined_simple(
                                            &mut attributes,
                                            key,
                                            undefined,
                                        );
                                    }
                                }
                            }
                            "t2" => {
                                let (new_input, simple_res) = crate::ast::SimpleAttri::nom_parse(
                                    input,
                                    scope,
                                )?;
                                input = new_input;
                                match simple_res {
                                    Ok(simple) => {
                                        t2 = Some(simple);
                                    }
                                    Err(undefined) => {
                                        {
                                            let lvl = ::log::Level::Error;
                                            if lvl <= ::log::STATIC_MAX_LEVEL
                                                && lvl <= ::log::max_level()
                                            {
                                                ::log::__private_api::log(
                                                    format_args!(
                                                        "Line={0}; Key={1}; Value={2:?}",
                                                        scope.line_num,
                                                        key,
                                                        undefined,
                                                    ),
                                                    lvl,
                                                    &(
                                                        "liberty_db::common::demo",
                                                        "liberty_db::common::demo",
                                                        ::log::__private_api::loc(),
                                                    ),
                                                    (),
                                                );
                                            }
                                        };
                                        crate::ast::attributs_set_undefined_simple(
                                            &mut attributes,
                                            key,
                                            undefined,
                                        );
                                    }
                                }
                            }
                            _ => {
                                let (new_input, undefined) = crate::ast::parser::undefine(
                                    input,
                                    key,
                                    scope,
                                )?;
                                input = new_input;
                                crate::ast::attributs_set_undefined_attri(
                                    &mut attributes,
                                    key,
                                    group_name,
                                    scope,
                                    undefined,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Timing {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "Timing",
                "attributes",
                &self.attributes,
                "comments",
                &self.comments,
                "values",
                &self.values,
                "t1",
                &self.t1,
                "t2",
                &&self.t2,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Timing {
        #[inline]
        fn clone(&self) -> Timing {
            Timing {
                attributes: ::core::clone::Clone::clone(&self.attributes),
                comments: ::core::clone::Clone::clone(&self.comments),
                values: ::core::clone::Clone::clone(&self.values),
                t1: ::core::clone::Clone::clone(&self.t1),
                t2: ::core::clone::Clone::clone(&self.t2),
            }
        }
    }
    impl GroupFn for Timing {}
}
