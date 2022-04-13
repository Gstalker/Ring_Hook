use anyhow::Error;
use std::sync::Mutex;
use jni::{
    JNIEnv,
    objects::{
        JObject,
        JClass
    },
};
use jni::objects::{GlobalRef};

pub fn load_dex_files(env: &mut JNIEnv, dex_files: Vec<Vec<u8>>) -> Result<(),Error>{
    DEX_LOADER.lock().unwrap().set_dex_files(dex_files);
    DEX_LOADER.lock().unwrap().load_dex_file(env)?;
    Ok(())
}

pub fn invoke_java_entry(env: &mut JNIEnv) {
    DEX_LOADER.lock().unwrap().invoke_java_entry(env);
}


struct DexLoader{
    dex_files: Vec<Vec<u8>>,
    class_loader: Option<GlobalRef>,
}

impl DexLoader {
    pub fn new() -> Option<Self> {
        let mut init_marker_guard = DEX_LOADER_INITIALIZED.lock().unwrap();
        let init_marker = *init_marker_guard;
        if init_marker {
            return None;
        }
        *init_marker_guard = true;
        Some(Self{
            dex_files: Vec::new(),
            class_loader: None
        })
    }

    pub fn set_dex_files(&mut self,dex_files: Vec<Vec<u8>>) {
        self.dex_files = dex_files;
    }

    pub fn load_dex_file(&mut self,env: &mut JNIEnv) -> Result<(), Error>{
        trace!("get class_loader");
        let system_class_loader :JObject = env.call_static_method(
            env.find_class("java/lang/ClassLoader")?,
            "getSystemClassLoader",
            "()Ljava/lang/ClassLoader;",
            &[])?
            .l()?;
        trace!("create dex_buffer_array");
        let dex_buffer_array = env.new_object_array(
            self.dex_files.len() as i32,
            env.find_class("java/nio/ByteBuffer")?,
            JObject::null()
        )?;

        trace!("setting dexfile into array");
        for i in 0..self.dex_files.len() {
            let dex_data: &mut Vec<u8> = self.dex_files.get_mut(i).unwrap();
            let dex_buffer = env.new_direct_byte_buffer(dex_data)?;
            env.set_object_array_element(dex_buffer_array, i.try_into()?, dex_buffer)?;
        }
        trace!("create hooker_classloader");

        let hooker_class_loader: JObject = env.new_object(
            env.find_class("dalvik/system/InMemoryDexClassLoader")?,
            "([Ljava/nio/ByteBuffer;Ljava/lang/ClassLoader;)V",
                &[dex_buffer_array.into(),system_class_loader.into()]
        )?;

        trace!("global pinning classloader");


        let hooker_class_loader_global_ref = env.new_global_ref(hooker_class_loader)?;
        self.class_loader = Some(hooker_class_loader_global_ref);

        trace!("delete local_ref");
        env.delete_local_ref(system_class_loader)?;
        env.delete_local_ref(hooker_class_loader)?;
        Ok(())
    }

    fn find_class_from_hooker_class_loader(&mut self, env: &mut JNIEnv, class_name: String) -> Result<JClass,Error> {
        trace!("find class from hooker_class loader: {}",class_name.clone());
        let class_loader_ptr = self.class_loader.as_ref().unwrap().as_obj();
        let target_class = env.call_method(
            class_loader_ptr,
            "loadClass",
            "(Ljava/lang/String;)Ljava/lang/Class;",
            &[env.new_string(class_name)?.into()]
        )?.l()?.into_inner().into();
        trace!("success!");
        Ok(target_class)
    }

    #[allow(dead_code)]
    pub fn invoke_java_entry(&mut self, env: &mut JNIEnv) {
        trace!("invoke java entry");
        let entry_point_class_name_guard = JAVA_ENTRY_POINT_CLASS_NAME.lock().unwrap();
        let name = String::from(&*entry_point_class_name_guard);
        let entry_point_class = self.find_class_from_hooker_class_loader(
            env,
            name
        ).unwrap();

        trace!("do invoke!");
        env.call_static_method(
            entry_point_class,
            "Init",
            "()V",
            &[]
        );
        trace!("java entry finished");

        #[allow(unused_variables)]
        let global_pinning = env.new_global_ref(entry_point_class);
        trace!("pin the entry class");
    }
}

lazy_static! {
    static ref JAVA_ENTRY_POINT_CLASS_NAME: Mutex<String> = Mutex::new(String::from("bin.gstalker.ring.RingEntry"));
    static ref DEX_LOADER_INITIALIZED: Mutex<bool> = Mutex::new(false);
    static ref DEX_LOADER: Mutex<DexLoader> = Mutex::new(DexLoader::new().unwrap());
}