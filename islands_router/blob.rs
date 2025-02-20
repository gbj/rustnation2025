#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod app {
    use leptos::prelude::*;
    use leptos_router::{
        components::{FlatRoutes, Route, Router},
        StaticSegment,
    };
    pub fn shell(options: LeptosOptions) -> impl IntoView {
        {
            #[allow(unused_braces)]
            {
                ::leptos::prelude::View::new((
                    ::leptos::tachys::html::doctype("html"),
                    ::leptos::tachys::html::element::html()
                        .child((
                            ::leptos::tachys::html::element::head()
                                .child((
                                    ::leptos::tachys::html::InertElement::new(
                                        "<meta charset=\"utf-8\">",
                                    ),
                                    ::leptos::tachys::html::InertElement::new(
                                        "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">",
                                    ),
                                    {
                                        #[allow(unreachable_code)] #[allow(unused_mut)]
                                        #[allow(clippy::let_and_return)]
                                        ::leptos::component::component_view(
                                            #[allow(clippy::needless_borrows_for_generic_args)]
                                            &AutoReload,
                                            {
                                                let mut props = ::leptos::component::component_props_builder(
                                                        &AutoReload,
                                                    )
                                                    .options(#[allow(unused_braces)] { options.clone() })
                                                    .build();
                                                props
                                            },
                                        )
                                    },
                                    {
                                        #[allow(unreachable_code)] #[allow(unused_mut)]
                                        #[allow(clippy::let_and_return)]
                                        ::leptos::component::component_view(
                                            #[allow(clippy::needless_borrows_for_generic_args)]
                                            &HydrationScripts,
                                            {
                                                let mut props = ::leptos::component::component_props_builder(
                                                        &HydrationScripts,
                                                    )
                                                    .options(#[allow(unused_braces)] { options })
                                                    .islands(#[allow(unused_braces)] { true })
                                                    .build();
                                                props
                                            },
                                        )
                                    },
                                    ::leptos::tachys::html::InertElement::new(
                                        "<link rel=\"stylesheet\" id=\"leptos\" href=\"/pkg/islands.css\">",
                                    ),
                                    ::leptos::tachys::html::InertElement::new(
                                        "<link rel=\"shortcut icon\" type=\"image/ico\" href=\"/favicon.ico\">",
                                    ),
                                )),
                            ::leptos::tachys::html::element::body()
                                .child(
                                    #[allow(unused_braces)]
                                    {
                                        {
                                            #[allow(unreachable_code)] #[allow(unused_mut)]
                                            #[allow(clippy::let_and_return)]
                                            ::leptos::component::component_view(
                                                #[allow(clippy::needless_borrows_for_generic_args)]
                                                &App,
                                                {
                                                    let mut props = ::leptos::component::component_props_builder(
                                                            &App,
                                                        )
                                                        .build();
                                                    props
                                                },
                                            )
                                        }
                                    },
                                ),
                        ))
                        .lang("en"),
                ))
            }
        }
    }
    /// Props for the [`App`] component.
    ///
    #[builder(crate_module_path = ::leptos::typed_builder)]
    #[allow(non_snake_case)]
    pub struct AppProps {}
    #[automatically_derived]
    impl AppProps {
        /**
                Create a builder for building `AppProps`.
                On the builder, call  to set the values of the fields.
                Finally, call `.build()` to create the instance of `AppProps`.
                */
        #[allow(dead_code, clippy::default_trait_access)]
        pub fn builder() -> AppPropsBuilder<()> {
            AppPropsBuilder {
                fields: (),
                phantom: ::core::default::Default::default(),
            }
        }
    }
    #[must_use]
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub struct AppPropsBuilder<TypedBuilderFields = ()> {
        fields: TypedBuilderFields,
        phantom: ::core::marker::PhantomData<()>,
    }
    #[automatically_derived]
    impl<TypedBuilderFields> Clone for AppPropsBuilder<TypedBuilderFields>
    where
        TypedBuilderFields: Clone,
    {
        #[allow(clippy::default_trait_access)]
        fn clone(&self) -> Self {
            Self {
                fields: self.fields.clone(),
                phantom: ::core::default::Default::default(),
            }
        }
    }
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    #[automatically_derived]
    impl AppPropsBuilder<()> {
        #[allow(
            clippy::default_trait_access,
            clippy::used_underscore_binding,
            clippy::no_effect_underscore_binding
        )]
        pub fn build(self) -> AppProps {
            let () = self.fields;
            #[allow(deprecated)] AppProps {}.into()
        }
    }
    #[allow(missing_docs)]
    impl ::leptos::component::Props for AppProps {
        type Builder = AppPropsBuilder;
        fn builder() -> Self::Builder {
            AppProps::builder()
        }
    }
    #[allow(non_snake_case, clippy::too_many_arguments)]
    #[allow(clippy::needless_lifetimes)]
    #[allow(clippy::let_with_type_underscore)]
    pub fn App() -> impl IntoView {
        {}
        #[allow(clippy::suspicious_else_formatting)]
        {
            let __tracing_attr_span;
            let __tracing_attr_guard;
            if () || () {
                __tracing_attr_span = ();
                __tracing_attr_guard = __tracing_attr_span.enter();
            }
            #[warn(clippy::suspicious_else_formatting)]
            {
                #[allow(
                    unknown_lints,
                    unreachable_code,
                    clippy::diverging_sub_expression,
                    clippy::let_unit_value,
                    clippy::unreachable,
                    clippy::let_with_type_underscore,
                    clippy::empty_loop
                )]
                if false {
                    let __tracing_attr_fake_return: _ = loop {};
                    return __tracing_attr_fake_return;
                }
                {
                    let __span = ::leptos::tracing::Span::current();
                    ::leptos::prelude::untrack(move || {
                        #[cfg(debug_assertions)]
                        let _guard = __span.entered();
                        __App()
                    })
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(
        non_snake_case,
        dead_code,
        clippy::too_many_arguments,
        clippy::needless_lifetimes
    )]
    pub fn __App() -> impl IntoView {
        {
            #[allow(unused_braces)]
            {
                ::leptos::prelude::View::new((
                    ::leptos::tachys::html::element::script().src("/routing.js"),
                    {
                        #[allow(unreachable_code)] #[allow(unused_mut)]
                        #[allow(clippy::let_and_return)]
                        ::leptos::component::component_view(
                            #[allow(clippy::needless_borrows_for_generic_args)]
                            &Router,
                            {
                                let mut props = ::leptos::component::component_props_builder(
                                        &Router,
                                    )
                                    .children({
                                        ::leptos::children::ToChildren::to_children(move || (
                                            ::leptos::tachys::html::element::header()
                                                .child(
                                                    #[allow(unused_braces)]
                                                    {
                                                        ::leptos::tachys::html::InertElement::new(
                                                            "<h1>My Application</h1>",
                                                        )
                                                    },
                                                ),
                                            ::leptos::tachys::html::element::nav()
                                                .child((
                                                    ::leptos::tachys::html::InertElement::new(
                                                        "<a href=\"/\">Page A</a>",
                                                    ),
                                                    ::leptos::tachys::html::InertElement::new(
                                                        "<a href=\"/b\">Page B</a>",
                                                    ),
                                                )),
                                            ::leptos::tachys::html::element::main()
                                                .child((
                                                    ::leptos::tachys::html::InertElement::new(
                                                        "<p><label>Home Checkbox<input type=\"checkbox\"></label></p>",
                                                    ),
                                                    {
                                                        #[allow(unreachable_code)] #[allow(unused_mut)]
                                                        #[allow(clippy::let_and_return)]
                                                        ::leptos::component::component_view(
                                                            #[allow(clippy::needless_borrows_for_generic_args)]
                                                            &FlatRoutes,
                                                            {
                                                                let mut props = ::leptos::component::component_props_builder(
                                                                        &FlatRoutes,
                                                                    )
                                                                    .fallback(#[allow(unused_braces)] { || "Not found." })
                                                                    .children({
                                                                        ::leptos::children::ToChildren::to_children(move || (
                                                                            {
                                                                                #[allow(unreachable_code)] #[allow(unused_mut)]
                                                                                #[allow(clippy::let_and_return)]
                                                                                ::leptos::component::component_view(
                                                                                    #[allow(clippy::needless_borrows_for_generic_args)]
                                                                                    &Route,
                                                                                    {
                                                                                        let mut props = ::leptos::component::component_props_builder(
                                                                                                &Route,
                                                                                            )
                                                                                            .path(#[allow(unused_braces)] { StaticSegment("") })
                                                                                            .view(#[allow(unused_braces)] { PageA })
                                                                                            .build();
                                                                                        props
                                                                                    },
                                                                                )
                                                                            },
                                                                            {
                                                                                #[allow(unreachable_code)] #[allow(unused_mut)]
                                                                                #[allow(clippy::let_and_return)]
                                                                                ::leptos::component::component_view(
                                                                                    #[allow(clippy::needless_borrows_for_generic_args)]
                                                                                    &Route,
                                                                                    {
                                                                                        let mut props = ::leptos::component::component_props_builder(
                                                                                                &Route,
                                                                                            )
                                                                                            .path(#[allow(unused_braces)] { StaticSegment("b") })
                                                                                            .view(#[allow(unused_braces)] { PageB })
                                                                                            .build();
                                                                                        props
                                                                                    },
                                                                                )
                                                                            },
                                                                        ))
                                                                    })
                                                                    .build();
                                                                props
                                                            },
                                                        )
                                                    },
                                                )),
                                        ))
                                    })
                                    .build();
                                props
                            },
                        )
                    },
                ))
            }
        }
    }
    /// Props for the [`PageA`] component.
    ///
    #[builder(crate_module_path = ::leptos::typed_builder)]
    #[allow(non_snake_case)]
    pub struct PageAProps {}
    #[automatically_derived]
    impl PageAProps {
        /**
                Create a builder for building `PageAProps`.
                On the builder, call  to set the values of the fields.
                Finally, call `.build()` to create the instance of `PageAProps`.
                */
        #[allow(dead_code, clippy::default_trait_access)]
        pub fn builder() -> PageAPropsBuilder<()> {
            PageAPropsBuilder {
                fields: (),
                phantom: ::core::default::Default::default(),
            }
        }
    }
    #[must_use]
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub struct PageAPropsBuilder<TypedBuilderFields = ()> {
        fields: TypedBuilderFields,
        phantom: ::core::marker::PhantomData<()>,
    }
    #[automatically_derived]
    impl<TypedBuilderFields> Clone for PageAPropsBuilder<TypedBuilderFields>
    where
        TypedBuilderFields: Clone,
    {
        #[allow(clippy::default_trait_access)]
        fn clone(&self) -> Self {
            Self {
                fields: self.fields.clone(),
                phantom: ::core::default::Default::default(),
            }
        }
    }
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    #[automatically_derived]
    impl PageAPropsBuilder<()> {
        #[allow(
            clippy::default_trait_access,
            clippy::used_underscore_binding,
            clippy::no_effect_underscore_binding
        )]
        pub fn build(self) -> PageAProps {
            let () = self.fields;
            #[allow(deprecated)] PageAProps {}.into()
        }
    }
    #[allow(missing_docs)]
    impl ::leptos::component::Props for PageAProps {
        type Builder = PageAPropsBuilder;
        fn builder() -> Self::Builder {
            PageAProps::builder()
        }
    }
    #[allow(non_snake_case, clippy::too_many_arguments)]
    #[allow(clippy::needless_lifetimes)]
    #[allow(clippy::let_with_type_underscore)]
    pub fn PageA() -> impl IntoView {
        {}
        #[allow(clippy::suspicious_else_formatting)]
        {
            let __tracing_attr_span;
            let __tracing_attr_guard;
            if () || () {
                __tracing_attr_span = ();
                __tracing_attr_guard = __tracing_attr_span.enter();
            }
            #[warn(clippy::suspicious_else_formatting)]
            {
                #[allow(
                    unknown_lints,
                    unreachable_code,
                    clippy::diverging_sub_expression,
                    clippy::let_unit_value,
                    clippy::unreachable,
                    clippy::let_with_type_underscore,
                    clippy::empty_loop
                )]
                if false {
                    let __tracing_attr_fake_return: _ = loop {};
                    return __tracing_attr_fake_return;
                }
                {
                    let __span = ::leptos::tracing::Span::current();
                    ::leptos::prelude::untrack(move || {
                        #[cfg(debug_assertions)]
                        let _guard = __span.entered();
                        __PageA()
                    })
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(
        non_snake_case,
        dead_code,
        clippy::too_many_arguments,
        clippy::needless_lifetimes
    )]
    pub fn __PageA() -> impl IntoView {
        {
            #[allow(unused_braces)]
            {
                ::leptos::prelude::View::new(
                    ::leptos::tachys::html::element::label()
                        .child((
                            "Page A",
                            ::leptos::tachys::html::InertElement::new(
                                "<input type=\"checkbox\">",
                            ),
                        )),
                )
            }
        }
    }
    /// Props for the [`PageB`] component.
    ///
    #[builder(crate_module_path = ::leptos::typed_builder)]
    #[allow(non_snake_case)]
    pub struct PageBProps {}
    #[automatically_derived]
    impl PageBProps {
        /**
                Create a builder for building `PageBProps`.
                On the builder, call  to set the values of the fields.
                Finally, call `.build()` to create the instance of `PageBProps`.
                */
        #[allow(dead_code, clippy::default_trait_access)]
        pub fn builder() -> PageBPropsBuilder<()> {
            PageBPropsBuilder {
                fields: (),
                phantom: ::core::default::Default::default(),
            }
        }
    }
    #[must_use]
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub struct PageBPropsBuilder<TypedBuilderFields = ()> {
        fields: TypedBuilderFields,
        phantom: ::core::marker::PhantomData<()>,
    }
    #[automatically_derived]
    impl<TypedBuilderFields> Clone for PageBPropsBuilder<TypedBuilderFields>
    where
        TypedBuilderFields: Clone,
    {
        #[allow(clippy::default_trait_access)]
        fn clone(&self) -> Self {
            Self {
                fields: self.fields.clone(),
                phantom: ::core::default::Default::default(),
            }
        }
    }
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    #[automatically_derived]
    impl PageBPropsBuilder<()> {
        #[allow(
            clippy::default_trait_access,
            clippy::used_underscore_binding,
            clippy::no_effect_underscore_binding
        )]
        pub fn build(self) -> PageBProps {
            let () = self.fields;
            #[allow(deprecated)] PageBProps {}.into()
        }
    }
    #[allow(missing_docs)]
    impl ::leptos::component::Props for PageBProps {
        type Builder = PageBPropsBuilder;
        fn builder() -> Self::Builder {
            PageBProps::builder()
        }
    }
    #[allow(non_snake_case, clippy::too_many_arguments)]
    #[allow(clippy::needless_lifetimes)]
    #[allow(clippy::let_with_type_underscore)]
    pub fn PageB() -> impl IntoView {
        {}
        #[allow(clippy::suspicious_else_formatting)]
        {
            let __tracing_attr_span;
            let __tracing_attr_guard;
            if () || () {
                __tracing_attr_span = ();
                __tracing_attr_guard = __tracing_attr_span.enter();
            }
            #[warn(clippy::suspicious_else_formatting)]
            {
                #[allow(
                    unknown_lints,
                    unreachable_code,
                    clippy::diverging_sub_expression,
                    clippy::let_unit_value,
                    clippy::unreachable,
                    clippy::let_with_type_underscore,
                    clippy::empty_loop
                )]
                if false {
                    let __tracing_attr_fake_return: _ = loop {};
                    return __tracing_attr_fake_return;
                }
                {
                    let __span = ::leptos::tracing::Span::current();
                    ::leptos::prelude::untrack(move || {
                        #[cfg(debug_assertions)]
                        let _guard = __span.entered();
                        __PageB()
                    })
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(
        non_snake_case,
        dead_code,
        clippy::too_many_arguments,
        clippy::needless_lifetimes
    )]
    pub fn __PageB() -> impl IntoView {
        {
            #[allow(unused_braces)]
            {
                ::leptos::prelude::View::new(
                    ::leptos::tachys::html::element::label()
                        .child((
                            "Page B",
                            ::leptos::tachys::html::InertElement::new(
                                "<input type=\"checkbox\">",
                            ),
                        )),
                )
            }
        }
    }
}
