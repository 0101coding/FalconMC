use quote::format_ident;
use syn::{parse_quote_spanned, spanned::Spanned, Expr, Stmt, Type};

use crate::attributes::PacketAttribute::{self, *};

pub fn to_preprocess(attribute: &PacketAttribute, field: Expr) -> Option<Stmt> {
    match attribute {
        Vec(data) => {
            let target = &data.target;
            Some(parse_quote_spanned! {field.span()=>
                let #target = #field.len();
            })
        }
        Bytes(data) => data.target.as_ref().map(|target| {
            parse_quote_spanned! {field.span()=>
                let #target = ::falcon_packet_core::PacketSizeSeed::size(
                    &::falcon_packet_core::AsRefU8::default(),
                    &#field,
                );
            }
        }),
        Link(data) => {
            let prefix = format_ident!("{}_value", data.prefix);
            let target = &data.target;
            let others = data.others.as_ref();
            Some(match others.map(|o| o.into_iter()) {
                Some(others) => parse_quote_spanned! {field.span()=>
                    let #target = #prefix(&#field, #(&self.#others),*);
                },
                None => parse_quote_spanned! {field.span()=>
                    let #target = #prefix(&#field);
                },
            })
        }
        _ => None,
    }
}

pub fn to_end(attribute: &PacketAttribute, field: Expr) -> Option<Expr> {
    match attribute {
        String(data) => {
            let len = &data.max_length;
            Some(parse_quote_spanned! {field.span()=>
                ::falcon_packet_core::PacketSizeSeed::size(
                    &::falcon_packet_core::PacketString::new(#len),
                    &#field,
                )
            })
        }
        Vec(_) => Some(parse_quote_spanned! {field.span()=>
            ::falcon_packet_core::PacketSizeSeed::size(
                &::falcon_packet_core::PacketVec::new(0),
                &#field,
            )
        }),
        Array(_) => Some(parse_quote_spanned! {field.span()=>
            ::falcon_packet_core::PacketSizeSeed::size(
                &::falcon_packet_core::PacketArray::default(),
                &#field,
            )
        }),
        Bytes(_) => Some(parse_quote_spanned! {field.span()=>
            ::falcon_packet_core::PacketSizeSeed::size(
                &::falcon_packet_core::AsRefU8::default(),
                &#field,
            )
        }),
        Link(data) => {
            let prefix = format_ident!("{}_size", data.prefix);
            Some(parse_quote_spanned! {field.span()=>
                #prefix(&#field)
            })
        }
        Nbt(_) => Some(parse_quote_spanned! {field.span()=>
            {
                let mut writer = ::falcon_packet_core::special::Counter::new();
                ::fastnbt::to_writer(&mut writer, &#field).expect("Invalid NBT to be sent!!");
                writer.count()
            }
        }),
        _ => None,
    }
}

pub fn to_tokenstream(attribute: &PacketAttribute, field: Expr, field_ty: &Type) -> Expr {
    match attribute {
        VarI32(_) => {
            parse_quote_spanned! {field.span()=>
                ::falcon_packet_core::VarI32::from(#field)
            }
        }
        VarI64(_) => {
            parse_quote_spanned! {field.span()=>
                ::falcon_packet_core::VarI64::from(#field)
            }
        }
        Into(data) => {
            let target = &data.target;
            parse_quote_spanned! {field.span()=>
                <#field_ty as ::std::convert::Into<#target>>::into(::std::clone::Clone::clone(&#field))
            }
        }
        Convert(data) => {
            let target = &data.target;
            parse_quote_spanned! {field.span()=>
                <#field_ty as ::std::convert::Into<#target>>::into(::std::clone::Clone::clone(&#field))
            }
        }
        _ => field,
    }
}
