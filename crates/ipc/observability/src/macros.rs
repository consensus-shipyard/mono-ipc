// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

#[macro_export]
macro_rules! register_metrics {
  ($($name:ident : $type:ty = $make:expr);* $(;)?) => {
      $(
        lazy_static! {
          pub static ref $name: $type = $make.unwrap();
        }
      )*

      pub fn register_metrics(registry: &Registry) -> anyhow::Result<()> {
        $(registry.register(Box::new($name.clone()))?;)*
        Ok(())
      }
  };
}

#[macro_export]
macro_rules! impl_traceable {
    ($struct_name:ident<$lifetime:tt>, $trace_level:expr, $domain:expr) => {
        impl<$lifetime> Traceable for $struct_name<$lifetime> {
            fn name() -> &'static str {
                stringify!($struct_name)
            }

            fn trace_level(&self) -> TraceLevel {
                $trace_level
            }

            fn domain(&self) -> &'static str {
                $domain
            }
        }
    };
    ($struct_name:ident, $trace_level:expr, $domain:expr) => {
        impl Traceable for $struct_name {
            fn name() -> &'static str {
                stringify!($struct_name)
            }

            fn trace_level(&self) -> TraceLevel {
                $trace_level
            }

            fn domain(&self) -> &'static str {
                $domain
            }
        }
    };
}

#[macro_export]
macro_rules! impl_traceables {
  ($trace_level:expr, $domain:expr, $($struct_name:ident$(<$lifetime:tt>)?),+) => {
      $(
          impl_traceable!($struct_name$(<$lifetime>)?, $trace_level, $domain);
      )+
  };
}
