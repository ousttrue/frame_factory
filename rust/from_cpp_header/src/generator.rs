use std::io::{BufWriter, Write};

use crate::{Args, Error, Type, TypeMap};

pub fn generate(type_map: &TypeMap, args: &Args)->Result<(), Error>
{
    std::fs::create_dir_all(args.dst.parent().unwrap()).unwrap();

    let mut f = BufWriter::new(std::fs::File::create(&args.dst).unwrap());     

    for (k, v) in type_map.get().iter()
    {
        if let Type::UserType(t) = &**v
        {
            if let Some(export) = args.find_export(&t.file)
            {
                f.write_fmt(format_args!("// {:?}\n", v));
            }
        }
    }

    Ok(())
}
