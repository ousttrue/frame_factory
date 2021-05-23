use std::{cell::RefCell, collections::HashMap, ffi::CStr, hash::Hash, ptr};

use com_ptr::ComPtr;
use winapi::um::{d3d11, d3d11shader};

use crate::com_util::{ComCreate, ComError};

struct Variable {
    offset: u32,
    length: u32,
}

pub struct ConstantBufferSlot {
    pub name: String,
    pub buffer: ComPtr<d3d11::ID3D11Buffer>,
    pub desc: d3d11::D3D11_BUFFER_DESC,
    pub back_buffer: RefCell<Vec<u8>>,
    variables: HashMap<String, Variable>,
}

impl ConstantBufferSlot {
    pub fn new(
        d3d_device: &d3d11::ID3D11Device,
        shader_desc: d3d11shader::D3D11_SHADER_BUFFER_DESC,
    ) -> Result<ConstantBufferSlot, ComError> {
        let desc = d3d11::D3D11_BUFFER_DESC {
            ByteWidth: shader_desc.Size,
            Usage: d3d11::D3D11_USAGE_DEFAULT,
            BindFlags: d3d11::D3D11_BIND_CONSTANT_BUFFER,
            ..Default::default()
        };

        let buffer = ComPtr::create_if_success(|pp| unsafe {
            d3d_device.CreateBuffer(&desc, std::ptr::null_mut(), pp)
        })?;

        let slot = ConstantBufferSlot {
            name: unsafe { String::from(CStr::from_ptr(shader_desc.Name).to_str().unwrap()) },
            buffer,
            desc,
            back_buffer: RefCell::new(Vec::with_capacity(desc.ByteWidth as usize)),
            variables: HashMap::new(),
        };
        unsafe {
            slot.back_buffer
                .borrow_mut()
                .set_len(desc.ByteWidth as usize)
        };

        Ok(slot)
    }

    fn update<T>(&self, key: &str, data: *const T) {
        if let Some(var) = self.variables.get(key) {
            if std::mem::size_of::<T>() > var.length as usize {
                panic!()
            };
            let src = data as *const u8;
            unsafe {
                let dst = self
                    .back_buffer
                    .borrow_mut()
                    .as_mut_ptr()
                    .offset(var.offset as isize);
                std::ptr::copy_nonoverlapping(src, dst, std::mem::size_of::<T>())
            }
        }
        else{
            let a=0;
        }
    }

    pub fn commit(&self, d3d_context: &d3d11::ID3D11DeviceContext) {
        let p: *mut d3d11::ID3D11Buffer = self.buffer.as_ptr();
        unsafe {
            d3d_context.UpdateSubresource(
                p as *mut d3d11::ID3D11Resource,
                0,
                ptr::null_mut(),
                self.back_buffer.borrow().as_ptr() as *const winapi::ctypes::c_void,
                0,
                0,
            )
        };
    }
}

pub struct ConstantBufferShader {
    pub slots: Vec<Box<ConstantBufferSlot>>,
}

impl ConstantBufferShader {
    pub unsafe fn new(
        d3d_device: &d3d11::ID3D11Device,
        reflection: &ComPtr<d3d11shader::ID3D11ShaderReflection>,
    ) -> Result<ConstantBufferShader, ComError> {
        let mut shader_desc = d3d11shader::D3D11_SHADER_DESC::default();
        reflection.GetDesc(&mut shader_desc);

        let mut constant_buffer = ConstantBufferShader { slots: vec![] };

        // analize constant buffer
        for i in 0..shader_desc.ConstantBuffers {
            let cb = reflection.GetConstantBufferByIndex(i).as_ref().unwrap();
            let mut desc = d3d11shader::D3D11_SHADER_BUFFER_DESC::default();
            cb.GetDesc(&mut desc);
            // println!(
            //     "[{}: {}] {} bytes",
            //     i,
            //     CStr::from_ptr(desc.Name).to_str().unwrap(),
            //     desc.Size
            // );

            let mut buffer = Box::new(ConstantBufferSlot::new(d3d_device, desc)?);

            for j in 0..desc.Variables {
                let v = cb.GetVariableByIndex(j).as_ref().unwrap();
                let mut var = d3d11shader::D3D11_SHADER_VARIABLE_DESC::default();
                v.GetDesc(&mut var);
                let name = CStr::from_ptr(var.Name).to_str().unwrap().to_owned();
                buffer.variables.insert(
                    name,
                    Variable {
                        offset: var.StartOffset,
                        length: var.Size,
                    },
                );
            }

            constant_buffer.slots.push(buffer);
        }

        Ok(constant_buffer)
    }

    pub fn update<T>(&self, slot_index: usize, key: &str, data: *const T) {
        if let Some(slot) = self.slots.get(slot_index) {
            slot.update(key, data);
        }
    }

    pub fn set_vs(&self, d3d_context: &d3d11::ID3D11DeviceContext) {
        for (i, slot) in self.slots.iter().enumerate() {
            unsafe {
                d3d_context.VSSetConstantBuffers(i as u32, 1, &mut slot.buffer.as_ptr());
            };
        }
    }
}
