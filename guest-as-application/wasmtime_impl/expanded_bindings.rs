mod bindings {
    use wasmtime::component::*;
    pub type I2c = wasi::i2c::i2c::I2c;
    /// Auto-generated bindings for a pre-instantiated version of a
    /// component which implements the world `pingpong`.
    ///
    /// This structure is created through [`PingpongPre::new`] which
    /// takes a [`InstancePre`](wasmtime::component::InstancePre) that
    /// has been created through a [`Linker`](wasmtime::component::Linker).
    ///
    /// For more information see [`Pingpong`] as well.
    pub struct PingpongPre<T: 'static> {
        instance_pre: wasmtime::component::InstancePre<T>,
        indices: PingpongIndices,
    }
    impl<T: 'static> Clone for PingpongPre<T> {
        fn clone(&self) -> Self {
            Self {
                instance_pre: self.instance_pre.clone(),
                indices: self.indices.clone(),
            }
        }
    }
    impl<_T: 'static> PingpongPre<_T> {
        /// Creates a new copy of `PingpongPre` bindings which can then
        /// be used to instantiate into a particular store.
        ///
        /// This method may fail if the component behind `instance_pre`
        /// does not have the required exports.
        pub fn new(
            instance_pre: wasmtime::component::InstancePre<_T>,
        ) -> wasmtime::Result<Self> {
            let indices = PingpongIndices::new(&instance_pre)?;
            Ok(Self { instance_pre, indices })
        }
        pub fn engine(&self) -> &wasmtime::Engine {
            self.instance_pre.engine()
        }
        pub fn instance_pre(&self) -> &wasmtime::component::InstancePre<_T> {
            &self.instance_pre
        }
        /// Instantiates a new instance of [`Pingpong`] within the
        /// `store` provided.
        ///
        /// This function will use `self` as the pre-instantiated
        /// instance to perform instantiation. Afterwards the preloaded
        /// indices in `self` are used to lookup all exports on the
        /// resulting instance.
        pub fn instantiate(
            &self,
            mut store: impl wasmtime::AsContextMut<Data = _T>,
        ) -> wasmtime::Result<Pingpong> {
            let mut store = store.as_context_mut();
            let instance = self.instance_pre.instantiate(&mut store)?;
            self.indices.load(&mut store, &instance)
        }
    }
    /// Auto-generated bindings for index of the exports of
    /// `pingpong`.
    ///
    /// This is an implementation detail of [`PingpongPre`] and can
    /// be constructed if needed as well.
    ///
    /// For more information see [`Pingpong`] as well.
    pub struct PingpongIndices {
        run: wasmtime::component::ComponentExportIndex,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PingpongIndices {
        #[inline]
        fn clone(&self) -> PingpongIndices {
            PingpongIndices {
                run: ::core::clone::Clone::clone(&self.run),
            }
        }
    }
    /// Auto-generated bindings for an instance a component which
    /// implements the world `pingpong`.
    ///
    /// This structure can be created through a number of means
    /// depending on your requirements and what you have on hand:
    ///
    /// * The most convenient way is to use
    ///   [`Pingpong::instantiate`] which only needs a
    ///   [`Store`], [`Component`], and [`Linker`].
    ///
    /// * Alternatively you can create a [`PingpongPre`] ahead of
    ///   time with a [`Component`] to front-load string lookups
    ///   of exports once instead of per-instantiation. This
    ///   method then uses [`PingpongPre::instantiate`] to
    ///   create a [`Pingpong`].
    ///
    /// * If you've instantiated the instance yourself already
    ///   then you can use [`Pingpong::new`].
    ///
    /// These methods are all equivalent to one another and move
    /// around the tradeoff of what work is performed when.
    ///
    /// [`Store`]: wasmtime::Store
    /// [`Component`]: wasmtime::component::Component
    /// [`Linker`]: wasmtime::component::Linker
    pub struct Pingpong {
        run: wasmtime::component::Func,
    }
    pub trait PingpongImports {
        fn get_i2c_bus(&mut self, bus_number: u32) -> wasmtime::component::Resource<I2c>;
    }
    impl<_T: PingpongImports + ?Sized> PingpongImports for &mut _T {
        fn get_i2c_bus(
            &mut self,
            bus_number: u32,
        ) -> wasmtime::component::Resource<I2c> {
            PingpongImports::get_i2c_bus(*self, bus_number)
        }
    }
    const _: () = {
        #[allow(unused_imports)]
        use wasmtime::component::__internal::anyhow;
        impl PingpongIndices {
            /// Creates a new copy of `PingpongIndices` bindings which can then
            /// be used to instantiate into a particular store.
            ///
            /// This method may fail if the component does not have the
            /// required exports.
            pub fn new<_T>(
                _instance_pre: &wasmtime::component::InstancePre<_T>,
            ) -> wasmtime::Result<Self> {
                let _component = _instance_pre.component();
                let _instance_type = _instance_pre.instance_type();
                let run = {
                    let (item, index) = _component
                        .get_export(None, "run")
                        .ok_or_else(|| ::anyhow::__private::must_use({
                            let error = ::anyhow::__private::format_err(
                                format_args!("no export `run` found"),
                            );
                            error
                        }))?;
                    match item {
                        wasmtime::component::types::ComponentItem::ComponentFunc(
                            func,
                        ) => {
                            anyhow::Context::context(
                                func.typecheck::<(), ()>(&_instance_type),
                                "type-checking export func `run`",
                            )?;
                            index
                        }
                        _ => {
                            Err(
                                ::anyhow::__private::must_use({
                                    let error = ::anyhow::__private::format_err(
                                        format_args!("export `run` is not a function"),
                                    );
                                    error
                                }),
                            )?
                        }
                    }
                };
                Ok(PingpongIndices { run })
            }
            /// Uses the indices stored in `self` to load an instance
            /// of [`Pingpong`] from the instance provided.
            ///
            /// Note that at this time this method will additionally
            /// perform type-checks of all exports.
            pub fn load(
                &self,
                mut store: impl wasmtime::AsContextMut,
                instance: &wasmtime::component::Instance,
            ) -> wasmtime::Result<Pingpong> {
                let _ = &mut store;
                let _instance = instance;
                let run = *_instance
                    .get_typed_func::<(), ()>(&mut store, &self.run)?
                    .func();
                Ok(Pingpong { run })
            }
        }
        impl Pingpong {
            /// Convenience wrapper around [`PingpongPre::new`] and
            /// [`PingpongPre::instantiate`].
            pub fn instantiate<_T>(
                store: impl wasmtime::AsContextMut<Data = _T>,
                component: &wasmtime::component::Component,
                linker: &wasmtime::component::Linker<_T>,
            ) -> wasmtime::Result<Pingpong> {
                let pre = linker.instantiate_pre(component)?;
                PingpongPre::new(pre)?.instantiate(store)
            }
            /// Convenience wrapper around [`PingpongIndices::new`] and
            /// [`PingpongIndices::load`].
            pub fn new(
                mut store: impl wasmtime::AsContextMut,
                instance: &wasmtime::component::Instance,
            ) -> wasmtime::Result<Pingpong> {
                let indices = PingpongIndices::new(&instance.instance_pre(&store))?;
                indices.load(&mut store, instance)
            }
            pub fn add_to_linker_imports<T, D>(
                linker: &mut wasmtime::component::Linker<T>,
                host_getter: fn(&mut T) -> D::Data<'_>,
            ) -> wasmtime::Result<()>
            where
                D: wasmtime::component::HasData,
                for<'a> D::Data<'a>: PingpongImports,
                T: 'static,
            {
                let mut linker = linker.root();
                linker
                    .func_wrap(
                        "get-i2c-bus",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (arg0,): (u32,)|
                        {
                            let host = &mut host_getter(caller.data_mut());
                            let r = PingpongImports::get_i2c_bus(host, arg0);
                            Ok((r,))
                        },
                    )?;
                Ok(())
            }
            pub fn add_to_linker<T, D>(
                linker: &mut wasmtime::component::Linker<T>,
                host_getter: fn(&mut T) -> D::Data<'_>,
            ) -> wasmtime::Result<()>
            where
                D: wasmtime::component::HasData,
                for<'a> D::Data<'a>: wasi::i2c::i2c::Host + PingpongImports,
                T: 'static,
            {
                Self::add_to_linker_imports::<T, D>(linker, host_getter)?;
                wasi::i2c::i2c::add_to_linker::<T, D>(linker, host_getter)?;
                Ok(())
            }
            pub fn call_run<S: wasmtime::AsContextMut>(
                &self,
                mut store: S,
            ) -> wasmtime::Result<()> {
                let callee = unsafe {
                    wasmtime::component::TypedFunc::<(), ()>::new_unchecked(self.run)
                };
                let () = callee.call(store.as_context_mut(), ())?;
                callee.post_return(store.as_context_mut())?;
                Ok(())
            }
        }
    };
    pub mod wasi {
        pub mod i2c {
            #[allow(clippy::all)]
            pub mod i2c {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::{anyhow, Box};
                pub type Address = u16;
                const _: () = {
                    if !(2 == <Address as wasmtime::component::ComponentType>::SIZE32) {
                        ::core::panicking::panic(
                            "assertion failed: 2 == <Address as wasmtime::component::ComponentType>::SIZE32",
                        )
                    }
                    if !(2 == <Address as wasmtime::component::ComponentType>::ALIGN32) {
                        ::core::panicking::panic(
                            "assertion failed: 2 == <Address as wasmtime::component::ComponentType>::ALIGN32",
                        )
                    }
                };
                #[component(enum)]
                #[repr(u8)]
                pub enum NoAcknowledgeSource {
                    #[component(name = "address")]
                    Address,
                    #[component(name = "data")]
                    Data,
                    #[component(name = "unknown")]
                    Unknown,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for NoAcknowledgeSource {
                    #[inline]
                    fn clone(&self) -> NoAcknowledgeSource {
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for NoAcknowledgeSource {}
                #[automatically_derived]
                impl ::core::cmp::Eq for NoAcknowledgeSource {
                    #[inline]
                    #[doc(hidden)]
                    #[coverage(off)]
                    fn assert_receiver_is_total_eq(&self) -> () {}
                }
                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for NoAcknowledgeSource {}
                #[automatically_derived]
                impl ::core::cmp::PartialEq for NoAcknowledgeSource {
                    #[inline]
                    fn eq(&self, other: &NoAcknowledgeSource) -> bool {
                        let __self_discr = ::core::intrinsics::discriminant_value(self);
                        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                        __self_discr == __arg1_discr
                    }
                }
                unsafe impl wasmtime::component::Lower for NoAcknowledgeSource {
                    #[inline]
                    fn linear_lower_to_flat<T>(
                        &self,
                        cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        dst: &mut core::mem::MaybeUninit<Self::Lower>,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Enum(i) => {
                                &cx.types[i]
                            }
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::MaybeUninitExt;
                                    let m: &mut core::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).tag)
                                }
                            }
                        }
                            .write(wasmtime::ValRaw::u32(*self as u32));
                        Ok(())
                    }
                    #[inline]
                    fn linear_lower_to_memory<T>(
                        &self,
                        cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        mut offset: usize,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Enum(i) => {
                                &cx.types[i]
                            }
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        if true {
                            if !(offset
                                % (<Self as wasmtime::component::ComponentType>::ALIGN32
                                    as usize) == 0)
                            {
                                ::core::panicking::panic(
                                    "assertion failed: offset % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                                )
                            }
                        }
                        let discrim = *self as u8;
                        *cx.get::<1>(offset) = discrim.to_le_bytes();
                        Ok(())
                    }
                }
                unsafe impl wasmtime::component::Lift for NoAcknowledgeSource {
                    #[inline]
                    fn linear_lift_from_flat(
                        cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        src: &Self::Lower,
                    ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Enum(i) => {
                                &cx.types[i]
                            }
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        let discrim = src.tag.get_u32();
                        if discrim >= 3 {
                            return ::anyhow::__private::Err({
                                let error = ::anyhow::__private::format_err(
                                    format_args!("unexpected discriminant: {0}", discrim),
                                );
                                error
                            });
                        }
                        Ok(unsafe {
                            wasmtime::component::__internal::transmute::<
                                u8,
                                NoAcknowledgeSource,
                            >(discrim as u8)
                        })
                    }
                    #[inline]
                    fn linear_lift_from_memory(
                        cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        bytes: &[u8],
                    ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                        let align = <Self as wasmtime::component::ComponentType>::ALIGN32;
                        if true {
                            if !((bytes.as_ptr() as usize) % (align as usize) == 0) {
                                ::core::panicking::panic(
                                    "assertion failed: (bytes.as_ptr() as usize) % (align as usize) == 0",
                                )
                            }
                        }
                        let discrim = bytes[0];
                        if discrim >= 3 {
                            return ::anyhow::__private::Err({
                                let error = ::anyhow::__private::format_err(
                                    format_args!("unexpected discriminant: {0}", discrim),
                                );
                                error
                            });
                        }
                        Ok(unsafe {
                            wasmtime::component::__internal::transmute::<
                                u8,
                                NoAcknowledgeSource,
                            >(discrim)
                        })
                    }
                }
                const _: () = {
                    #[doc(hidden)]
                    #[repr(C)]
                    pub struct LowerNoAcknowledgeSource {
                        tag: wasmtime::ValRaw,
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for LowerNoAcknowledgeSource {
                        #[inline]
                        fn clone(&self) -> LowerNoAcknowledgeSource {
                            let _: ::core::clone::AssertParamIsClone<wasmtime::ValRaw>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for LowerNoAcknowledgeSource {}
                    unsafe impl wasmtime::component::ComponentType
                    for NoAcknowledgeSource {
                        type Lower = LowerNoAcknowledgeSource;
                        #[inline]
                        fn typecheck(
                            ty: &wasmtime::component::__internal::InterfaceType,
                            types: &wasmtime::component::__internal::InstanceType<'_>,
                        ) -> wasmtime::component::__internal::anyhow::Result<()> {
                            wasmtime::component::__internal::typecheck_enum(
                                ty,
                                types,
                                &["address", "data", "unknown"],
                            )
                        }
                        const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::enum_(
                            3usize,
                        );
                    }
                    unsafe impl wasmtime::component::__internal::ComponentVariant
                    for NoAcknowledgeSource {
                        const CASES: &'static [Option<
                            wasmtime::component::__internal::CanonicalAbiInfo,
                        >] = &[None, None, None];
                    }
                };
                impl core::fmt::Debug for NoAcknowledgeSource {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        match self {
                            NoAcknowledgeSource::Address => {
                                f.debug_tuple("NoAcknowledgeSource::Address").finish()
                            }
                            NoAcknowledgeSource::Data => {
                                f.debug_tuple("NoAcknowledgeSource::Data").finish()
                            }
                            NoAcknowledgeSource::Unknown => {
                                f.debug_tuple("NoAcknowledgeSource::Unknown").finish()
                            }
                        }
                    }
                }
                const _: () = {
                    if !(1
                        == <NoAcknowledgeSource as wasmtime::component::ComponentType>::SIZE32)
                    {
                        ::core::panicking::panic(
                            "assertion failed: 1 == <NoAcknowledgeSource as wasmtime::component::ComponentType>::SIZE32",
                        )
                    }
                    if !(1
                        == <NoAcknowledgeSource as wasmtime::component::ComponentType>::ALIGN32)
                    {
                        ::core::panicking::panic(
                            "assertion failed: 1 == <NoAcknowledgeSource as wasmtime::component::ComponentType>::ALIGN32",
                        )
                    }
                };
                #[component(variant)]
                pub enum ErrorCode {
                    #[component(name = "bus")]
                    Bus,
                    #[component(name = "arbitration-loss")]
                    ArbitrationLoss,
                    #[component(name = "no-acknowledge")]
                    NoAcknowledge(NoAcknowledgeSource),
                    #[component(name = "overrun")]
                    Overrun,
                    #[component(name = "other")]
                    Other,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for ErrorCode {
                    #[inline]
                    fn clone(&self) -> ErrorCode {
                        let _: ::core::clone::AssertParamIsClone<NoAcknowledgeSource>;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for ErrorCode {}
                unsafe impl wasmtime::component::Lower for ErrorCode {
                    #[inline]
                    fn linear_lower_to_flat<T>(
                        &self,
                        cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        dst: &mut core::mem::MaybeUninit<Self::Lower>,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Variant(
                                i,
                            ) => &cx.types[i],
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        match self {
                            Self::Bus => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::MaybeUninitExt;
                                            let m: &mut core::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(0u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).Bus)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                            Self::ArbitrationLoss => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::MaybeUninitExt;
                                            let m: &mut core::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(1u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).ArbitrationLoss)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                            Self::NoAcknowledge(value) => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::MaybeUninitExt;
                                            let m: &mut core::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(2u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).NoAcknowledge)
                                                }
                                            }
                                        },
                                        |dst| {
                                            value
                                                .linear_lower_to_flat(
                                                    cx,
                                                    ty
                                                        .cases[2usize]
                                                        .unwrap_or_else(
                                                            wasmtime::component::__internal::bad_type_info,
                                                        ),
                                                    dst,
                                                )
                                        },
                                    )
                                }
                            }
                            Self::Overrun => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::MaybeUninitExt;
                                            let m: &mut core::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(3u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).Overrun)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                            Self::Other => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::MaybeUninitExt;
                                            let m: &mut core::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(4u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).Other)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                        }
                    }
                    #[inline]
                    fn linear_lower_to_memory<T>(
                        &self,
                        cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        mut offset: usize,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Variant(
                                i,
                            ) => &cx.types[i],
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        if true {
                            if !(offset
                                % (<Self as wasmtime::component::ComponentType>::ALIGN32
                                    as usize) == 0)
                            {
                                ::core::panicking::panic(
                                    "assertion failed: offset % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                                )
                            }
                        }
                        match self {
                            Self::Bus => {
                                *cx.get::<1usize>(offset) = 0u8.to_le_bytes();
                                Ok(())
                            }
                            Self::ArbitrationLoss => {
                                *cx.get::<1usize>(offset) = 1u8.to_le_bytes();
                                Ok(())
                            }
                            Self::NoAcknowledge(value) => {
                                *cx.get::<1usize>(offset) = 2u8.to_le_bytes();
                                value
                                    .linear_lower_to_memory(
                                        cx,
                                        ty
                                            .cases[2usize]
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        offset
                                            + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                    )
                            }
                            Self::Overrun => {
                                *cx.get::<1usize>(offset) = 3u8.to_le_bytes();
                                Ok(())
                            }
                            Self::Other => {
                                *cx.get::<1usize>(offset) = 4u8.to_le_bytes();
                                Ok(())
                            }
                        }
                    }
                }
                unsafe impl wasmtime::component::Lift for ErrorCode {
                    #[inline]
                    fn linear_lift_from_flat(
                        cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        src: &Self::Lower,
                    ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Variant(
                                i,
                            ) => &cx.types[i],
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        Ok(
                            match src.tag.get_u32() {
                                0u32 => Self::Bus,
                                1u32 => Self::ArbitrationLoss,
                                2u32 => {
                                    Self::NoAcknowledge(
                                        <NoAcknowledgeSource as wasmtime::component::Lift>::linear_lift_from_flat(
                                            cx,
                                            ty
                                                .cases[2usize]
                                                .unwrap_or_else(
                                                    wasmtime::component::__internal::bad_type_info,
                                                ),
                                            unsafe { &src.payload.NoAcknowledge },
                                        )?,
                                    )
                                }
                                3u32 => Self::Overrun,
                                4u32 => Self::Other,
                                discrim => {
                                    return ::anyhow::__private::Err(
                                        ::anyhow::Error::msg(
                                            ::alloc::__export::must_use({
                                                ::alloc::fmt::format(
                                                    format_args!("unexpected discriminant: {0}", discrim),
                                                )
                                            }),
                                        ),
                                    );
                                }
                            },
                        )
                    }
                    #[inline]
                    fn linear_lift_from_memory(
                        cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        bytes: &[u8],
                    ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                        let align = <Self as wasmtime::component::ComponentType>::ALIGN32;
                        if true {
                            if !((bytes.as_ptr() as usize) % (align as usize) == 0) {
                                ::core::panicking::panic(
                                    "assertion failed: (bytes.as_ptr() as usize) % (align as usize) == 0",
                                )
                            }
                        }
                        let discrim = bytes[0];
                        let payload_offset = <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32;
                        let payload = &bytes[payload_offset..];
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Variant(
                                i,
                            ) => &cx.types[i],
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        Ok(
                            match discrim {
                                0u8 => Self::Bus,
                                1u8 => Self::ArbitrationLoss,
                                2u8 => {
                                    Self::NoAcknowledge(
                                        <NoAcknowledgeSource as wasmtime::component::Lift>::linear_lift_from_memory(
                                            cx,
                                            ty
                                                .cases[2usize]
                                                .unwrap_or_else(
                                                    wasmtime::component::__internal::bad_type_info,
                                                ),
                                            &payload[..<NoAcknowledgeSource as wasmtime::component::ComponentType>::SIZE32],
                                        )?,
                                    )
                                }
                                3u8 => Self::Overrun,
                                4u8 => Self::Other,
                                discrim => {
                                    return ::anyhow::__private::Err(
                                        ::anyhow::Error::msg(
                                            ::alloc::__export::must_use({
                                                ::alloc::fmt::format(
                                                    format_args!("unexpected discriminant: {0}", discrim),
                                                )
                                            }),
                                        ),
                                    );
                                }
                            },
                        )
                    }
                }
                const _: () = {
                    #[doc(hidden)]
                    #[repr(C)]
                    pub struct LowerErrorCode<T2: Copy> {
                        tag: wasmtime::ValRaw,
                        payload: LowerPayloadErrorCode<T2>,
                    }
                    #[automatically_derived]
                    impl<T2: ::core::clone::Clone + Copy> ::core::clone::Clone
                    for LowerErrorCode<T2> {
                        #[inline]
                        fn clone(&self) -> LowerErrorCode<T2> {
                            LowerErrorCode {
                                tag: ::core::clone::Clone::clone(&self.tag),
                                payload: ::core::clone::Clone::clone(&self.payload),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<T2: ::core::marker::Copy + Copy> ::core::marker::Copy
                    for LowerErrorCode<T2> {}
                    #[doc(hidden)]
                    #[allow(non_snake_case)]
                    #[repr(C)]
                    union LowerPayloadErrorCode<T2: Copy> {
                        Bus: [wasmtime::ValRaw; 0],
                        ArbitrationLoss: [wasmtime::ValRaw; 0],
                        NoAcknowledge: T2,
                        Overrun: [wasmtime::ValRaw; 0],
                        Other: [wasmtime::ValRaw; 0],
                    }
                    #[automatically_derived]
                    #[allow(non_snake_case)]
                    impl<
                        T2: ::core::marker::Copy + ::core::clone::Clone + Copy,
                    > ::core::clone::Clone for LowerPayloadErrorCode<T2> {
                        #[inline]
                        fn clone(&self) -> LowerPayloadErrorCode<T2> {
                            let _: ::core::clone::AssertParamIsCopy<Self>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    #[allow(non_snake_case)]
                    impl<T2: ::core::marker::Copy + Copy> ::core::marker::Copy
                    for LowerPayloadErrorCode<T2> {}
                    unsafe impl wasmtime::component::ComponentType for ErrorCode {
                        type Lower = LowerErrorCode<
                            <NoAcknowledgeSource as wasmtime::component::ComponentType>::Lower,
                        >;
                        #[inline]
                        fn typecheck(
                            ty: &wasmtime::component::__internal::InterfaceType,
                            types: &wasmtime::component::__internal::InstanceType<'_>,
                        ) -> wasmtime::component::__internal::anyhow::Result<()> {
                            wasmtime::component::__internal::typecheck_variant(
                                ty,
                                types,
                                &[
                                    ("bus", None),
                                    ("arbitration-loss", None),
                                    (
                                        "no-acknowledge",
                                        Some(
                                            <NoAcknowledgeSource as wasmtime::component::ComponentType>::typecheck,
                                        ),
                                    ),
                                    ("overrun", None),
                                    ("other", None),
                                ],
                            )
                        }
                        const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::variant_static(
                            &[
                                None,
                                None,
                                Some(
                                    <NoAcknowledgeSource as wasmtime::component::ComponentType>::ABI,
                                ),
                                None,
                                None,
                            ],
                        );
                    }
                    unsafe impl wasmtime::component::__internal::ComponentVariant
                    for ErrorCode {
                        const CASES: &'static [Option<
                            wasmtime::component::__internal::CanonicalAbiInfo,
                        >] = &[
                            None,
                            None,
                            Some(
                                <NoAcknowledgeSource as wasmtime::component::ComponentType>::ABI,
                            ),
                            None,
                            None,
                        ];
                    }
                };
                impl core::fmt::Debug for ErrorCode {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        match self {
                            ErrorCode::Bus => f.debug_tuple("ErrorCode::Bus").finish(),
                            ErrorCode::ArbitrationLoss => {
                                f.debug_tuple("ErrorCode::ArbitrationLoss").finish()
                            }
                            ErrorCode::NoAcknowledge(e) => {
                                f.debug_tuple("ErrorCode::NoAcknowledge").field(e).finish()
                            }
                            ErrorCode::Overrun => {
                                f.debug_tuple("ErrorCode::Overrun").finish()
                            }
                            ErrorCode::Other => {
                                f.debug_tuple("ErrorCode::Other").finish()
                            }
                        }
                    }
                }
                impl core::fmt::Display for ErrorCode {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        f.write_fmt(format_args!("{0:?}", self))
                    }
                }
                impl core::error::Error for ErrorCode {}
                const _: () = {
                    if !(2 == <ErrorCode as wasmtime::component::ComponentType>::SIZE32)
                    {
                        ::core::panicking::panic(
                            "assertion failed: 2 == <ErrorCode as wasmtime::component::ComponentType>::SIZE32",
                        )
                    }
                    if !(1 == <ErrorCode as wasmtime::component::ComponentType>::ALIGN32)
                    {
                        ::core::panicking::panic(
                            "assertion failed: 1 == <ErrorCode as wasmtime::component::ComponentType>::ALIGN32",
                        )
                    }
                };
                pub enum I2c {}
                pub trait HostI2c {
                    fn read(
                        &mut self,
                        self_: wasmtime::component::Resource<I2c>,
                        address: Address,
                        len: u64,
                    ) -> Result<wasmtime::component::__internal::Vec<u8>, ErrorCode>;
                    fn write(
                        &mut self,
                        self_: wasmtime::component::Resource<I2c>,
                        address: Address,
                        data: wasmtime::component::__internal::Vec<u8>,
                    ) -> Result<(), ErrorCode>;
                    fn drop(
                        &mut self,
                        rep: wasmtime::component::Resource<I2c>,
                    ) -> wasmtime::Result<()>;
                }
                impl<_T: HostI2c + ?Sized> HostI2c for &mut _T {
                    fn read(
                        &mut self,
                        self_: wasmtime::component::Resource<I2c>,
                        address: Address,
                        len: u64,
                    ) -> Result<wasmtime::component::__internal::Vec<u8>, ErrorCode> {
                        HostI2c::read(*self, self_, address, len)
                    }
                    fn write(
                        &mut self,
                        self_: wasmtime::component::Resource<I2c>,
                        address: Address,
                        data: wasmtime::component::__internal::Vec<u8>,
                    ) -> Result<(), ErrorCode> {
                        HostI2c::write(*self, self_, address, data)
                    }
                    fn drop(
                        &mut self,
                        rep: wasmtime::component::Resource<I2c>,
                    ) -> wasmtime::Result<()> {
                        HostI2c::drop(*self, rep)
                    }
                }
                pub trait Host: HostI2c {}
                impl<_T: Host + ?Sized> Host for &mut _T {}
                pub fn add_to_linker<T, D>(
                    linker: &mut wasmtime::component::Linker<T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: wasmtime::component::HasData,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    let mut inst = linker.instance("wasi:i2c/i2c")?;
                    inst.resource(
                        "i2c",
                        wasmtime::component::ResourceType::host::<I2c>(),
                        move |mut store, rep| -> wasmtime::Result<()> {
                            HostI2c::drop(
                                &mut host_getter(store.data_mut()),
                                wasmtime::component::Resource::new_own(rep),
                            )
                        },
                    )?;
                    inst.func_wrap(
                        "[method]i2c.read",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                                arg2,
                            ): (wasmtime::component::Resource<I2c>, Address, u64)|
                        {
                            let host = &mut host_getter(caller.data_mut());
                            let r = HostI2c::read(host, arg0, arg1, arg2);
                            Ok((r,))
                        },
                    )?;
                    inst.func_wrap(
                        "[method]i2c.write",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                                arg2,
                            ): (
                                wasmtime::component::Resource<I2c>,
                                Address,
                                wasmtime::component::__internal::Vec<u8>,
                            )|
                        {
                            let host = &mut host_getter(caller.data_mut());
                            let r = HostI2c::write(host, arg0, arg1, arg2);
                            Ok((r,))
                        },
                    )?;
                    Ok(())
                }
            }
        }
    }
    const _: &str = "package wasi:i2c;\n\nworld imports {\n    import i2c;\n}\n";
    const _: &str = "package my:pingpong;\n\nworld pingpong {\n\timport wasi:i2c/i2c;\n\tuse wasi:i2c/i2c.{i2c};\n\n\timport get-i2c-bus: func(bus-number: u32) -> i2c;\n\n\texport run: func();\n}";
    const _: &str = "package wasi:i2c;\n\ninterface i2c {\n    type address = u16;\n    variant error-code {\n        bus,\n\n        arbitration-loss,\n\n        no-acknowledge(no-acknowledge-source),\n\n        overrun,\n\n        other,\n    }\n\n    enum no-acknowledge-source {\n        address,\n\n        data,\n\n        unknown,\n    }\n\n    resource i2c {\n\n        read: func(address: address, len: u64) -> result<list<u8>, error-code>;\n\n        write: func(address: address, data: list<u8>) -> result<_, error-code>;\n    }\n}\n";
}
