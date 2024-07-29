/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use proc_macro2::TokenStream;
use quote::quote;

use crate::ParseResult;

pub struct RpcFuncDefinition {
    pub method_name: String,
    pub mode: Option<TokenStream>,
    pub call_local: Option<TokenStream>,
    pub transfer_mode: Option<TokenStream>,
    pub transfer_channel: Option<TokenStream>,
}

pub fn make_rpc_registration(rpc_func_definition: RpcFuncDefinition) -> ParseResult<TokenStream> {
    let rpc_method_name = rpc_func_definition.method_name;

    let mode = rpc_func_definition.mode.map(|mode| {
        quote! {
            mode: #mode,
        }
    });

    let transfer_mode = rpc_func_definition.transfer_mode.map(|transfer_mode| {
        quote! {
            transfer_mode: #transfer_mode,
        }
    });

    let call_local = rpc_func_definition.call_local.map(|call_local| {
        quote! {
            call_local: #call_local,
        }
    });

    let transfer_channel = rpc_func_definition.transfer_channel.map(|channel| {
        quote! {
            transfer_channel: #channel,
        }
    });

    let registration = quote! {
        let rpc_configuration = ::godot::obj::RpcArgs {
            #mode
            #transfer_mode
            #call_local
            #transfer_channel
            ..Default::default()
        }.into_dictionary();

        base.rpc_config(#rpc_method_name.into(), rpc_configuration.to_variant());
    };

    Ok(registration)
}
