var searchIndex = JSON.parse('{\
"slos":{"doc":"","t":[3,3,4,13,6,13,13,11,11,11,11,11,11,0,5,11,11,0,11,11,11,11,11,11,5,11,11,11,11,11,5,11,11,11,11,11,11,11,11,11,11,12,3,7,17,17,7,11,11,11,11,5,11,11,5,5,11,11,11,3,11,11,11,0,5,11,5,11,11,11,11,11,3,3,11,11,11,11,12,11,11,11,11,11,11,12,11,11,11,11,11,11,12,11,11,11,11,11,11,5,11,11,11,11,11,11,11,11,11,11,11],"n":["KMAIN_INIT_PARTIALS","KMAIN_LOOP_PARTIALS","KernelError","KernelUninitialized","KmainPartial","MountError","Unknown","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","clock","current_system","deref","deref","filesystem","fmt","fmt","from","from","from","from","init","initialize","initialize","into","into","into","kmain","to_string","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","0","BOOT_CLOCK","CLOCK_TICK_WARN_COUNT","CLOCK_TICK_WARN_MS","CLOCK_TICK_WARN_THRESHOLD","CLOCK_UNSTABLE","borrow","borrow_mut","deref","from","init","initialize","into","on_tick","treat_as_unstable","try_from","try_into","type_id","FSBASE","borrow","borrow_mut","deref","devices","fopen","from","init","initialize","into","try_from","try_into","type_id","SystemDeviceCollection","SystemDeviceFile","borrow","borrow","borrow_mut","borrow_mut","devices","fmt","fmt","from","from","inode","inode","inode","into","into","mount","mount","name","name","name","new","open","permissions","permissions","push","readdir","system_devices","touch","try_directory","try_directory","try_file","try_file","try_from","try_from","try_into","try_into","type_id","type_id"],"q":["slos","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","slos::KernelError","slos::clock","","","","","","","","","","","","","","","","","slos::filesystem","","","","","","","","","","","","","slos::filesystem::devices","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["Collection of <code>KmainPartial</code> callbacks, called before the …","Collection of <code>KmainPartial</code> callbacks, called each …","Fatal kernel errors","<code>kmain</code> called before <code>init</code>","Callback type for <code>kmain</code> inner functions","Bubbled filesystem mount error","Unknown error","","","","","","","Kernel timekeeping","Returns the <code>SystemHardware</code> implementation for the running …","","","Global filesystem handling","","","","","","","Initializes the system.","","","","","","Kernel main function","","","","","","","","","","","","Time since system boot","Current number of warnings","Warn if the clock increases more than this time (in …","If <code>CLOCK_TICK_WARN_COUNT</code> &gt;= this value, set <code>CLOCK_UNSTABLE</code>","Whether to treat the system clock as unstable","","","","","Initialize the system timers","","","Tick the clock","Set the system clock as unstable","","","","Filesystem base used by the kernel","","","","Filesystem device node helpers","Open a file somewhere on the filesystem, returning a …","","Initialize the filesystem handlers","","","","","","Collection for core system device nodes to be presented …","A system device node to be presented to the filesystem","","","","","Devices to present to the filesystem","","","","","","","File inode","","","","","","","File name","","","","","","","Return a populated <code>SystemDeviceCollection</code>","","","","","","","","","","",""],"i":[0,0,0,1,0,1,1,1,2,3,1,2,3,0,0,2,3,0,1,1,1,1,2,3,0,2,3,1,2,3,0,1,1,2,3,1,2,3,1,2,3,4,0,0,0,0,0,5,5,5,5,0,5,5,0,0,5,5,5,0,6,6,6,0,0,6,0,6,6,6,6,6,0,0,7,8,7,8,8,7,8,7,8,7,8,7,7,8,7,8,7,8,7,8,7,7,8,8,8,0,8,7,8,7,8,7,8,7,8,7,8],"f":[null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],null,[[],["systemhardware",8]],[[],["staticcollection",3]],[[],["staticcollection",3]],null,[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["mounterror",4]],["kernelerror",4]],[[]],[[]],[[]],[[["systemhardware",8]],[["result",4,["kernelerror"]],["kernelerror",4]]],[[]],[[]],[[]],[[]],[[]],[[],[["result",4,["kernelerror"]],["kernelerror",4]]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,null,null,null,[[]],[[]],[[],["unsafecontainer",3]],[[]],[[],[["result",4,["kernelerror"]],["kernelerror",4]]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,[[]],[[]],[[],["unsafecontainer",3]],null,[[["str",15]],[["result",4,["fsfilehandle","fserror"]],["fserror",4],["fsfilehandle",8]]],[[]],[[],[["result",4,["kernelerror"]],["kernelerror",4]]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,[[]],[[]],[[]],[[]],null,[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[],["usize",15]],[[],["usize",15]],null,[[]],[[]],[[],[["option",4,["fsroot"]],["fsroot",8]]],[[],[["option",4,["fsroot"]],["fsroot",8]]],[[],["str",15]],[[],["str",15]],null,[[]],[[],[["fsfilehandle",8],["fserror",4],["result",4,["fsfilehandle","fserror"]]]],[[],["u16",15]],[[],["u16",15]],[[["systemdevicefile",3]]],[[],[["result",4,["vec","fserror"]],["vec",3,["fsnode"]],["fserror",4]]],[[],["systemdevicecollection",3]],[[["str",15]],[["result",4,["fsnode","fserror"]],["fsnode",8],["fserror",4]]],[[],[["option",4,["fsdirectory"]],["fsdirectory",8]]],[[],[["option",4,["fsdirectory"]],["fsdirectory",8]]],[[],[["fsfile",8],["option",4,["fsfile"]]]],[[],[["fsfile",8],["option",4,["fsfile"]]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]]],"p":[[4,"KernelError"],[3,"KMAIN_INIT_PARTIALS"],[3,"KMAIN_LOOP_PARTIALS"],[13,"MountError"],[3,"BOOT_CLOCK"],[3,"FSBASE"],[3,"SystemDeviceFile"],[3,"SystemDeviceCollection"]]},\
"slos_filesystem":{"doc":"","t":[13,13,13,3,3,13,8,4,8,8,8,8,8,8,13,4,13,13,13,13,13,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,11,11,11,11,0,10,11,12,10,11,11,10,0,12,11,11,10,10,10,10,12,11,11,10,10,10,11,11,11,11,11,11,11,11,11,11,11,11,12,3,3,11,11,11,11,12,12,11,11,11,12,11,11,11,11,11,11,12,11,11,11,11,11,11,12,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,17,5,5,5],"n":["EndOfFile","FileExists","FileNotFound","FilesystemBase","FilesystemMountpoint","FilesystemRootError","FsDirectory","FsError","FsFile","FsFileHandle","FsNode","FsReadDir","FsRoot","FsWriteDir","InvalidArgument","MountError","OpenHandleExists","PermissionDenied","PermissionDenied","StdIoError","Unknown","Unknown","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","default","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","inode","into","into","into","into","memory","mount","mount","mountpoints","name","new","node_at_path","open","path","path","path_string","path_vec","permissions","raw_read","raw_write","readdir","root","to_string","to_string","touch","try_directory","try_file","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","0","SimpleMemoryFs","SimpleMemoryFsFile","borrow","borrow","borrow_mut","borrow_mut","content","current_inode","default","eq","eq","files","fmt","fmt","from","from","inode","inode","inode","into","into","mount","mount","name","name","name","ne","ne","new","open","parent_index","permissions","permissions","raw_read","raw_write","readdir","touch","try_directory","try_directory","try_file","try_file","try_from","try_from","try_into","try_into","type_id","type_id","PATH_SEPARATOR","join","normalize","split"],"q":["slos_filesystem","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","slos_filesystem::FsError","slos_filesystem::memory","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","slos_filesystem::path","","",""],"d":["End of file reached (possibly unexpectedly)","A file with a duplicate name already exists","File not found","Base structure for mounting filesystems to","Container for filesystem mountpoint roots","Filesystem root error (possibly set to None)","A directory on a filesystem","File and directory errors","A file on a filesystem","Read/write handle to a <code>FsFile</code>","Filesystem node","Directory read functions","Mountable filesystem root","Directory write functions","Invalid argument","Mountpoint errors","File already has an open handle","Permission denied","Permission denied","IO error: {0}","Unknown error","Unknown error","","","","","","","","","","","","","","","","","","","","","Get the inode value for this node","","","","","SimpleMemoryFs - an in-memory single-directory read/write …","Try to get the root filesystem this node belongs to","Mount a filesystem root at <code>path</code>.","Collection of mounted filesystems","Get the filename of this node","Create a new empty <code>FilesystemBase</code> instance.","Return an <code>FsNode</code> for the given <code>path</code>, if one exists.","","Path normalization and manipulation","Segments of the path making up the mountpoint","Get the absolute path to this mountpoint as a String","Get the absolute path to this mountpoint","Get the permissions of this node","Try to read from the file","Try to write to the file","Return an <code>FsNode</code> reference for each node in this directory","Filesystem root, as a contained <code>FsRoot</code> trait object","","","Create a new empty file in this directory","Try to get this node as a <code>FsDirectory</code> trait object …","Try to get this node as a <code>FsFile</code> trait object reference","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Path separator character","Join a slice of path segments into a usable path","Normalize a given path","Split a path into it’s segments"],"i":[1,1,1,0,0,1,0,0,0,0,0,0,0,0,1,0,1,1,2,1,1,2,1,2,3,4,1,2,3,4,3,1,1,2,2,3,4,1,1,2,3,4,5,1,2,3,4,0,5,4,4,5,4,4,6,0,3,3,3,5,7,7,8,3,1,2,9,5,5,1,2,3,4,1,2,3,4,1,2,3,4,10,0,0,11,12,11,12,12,11,12,11,12,11,11,12,11,12,11,12,12,11,12,11,12,11,12,12,11,12,11,12,12,11,12,12,12,11,11,11,12,11,12,11,12,11,12,11,12,0,0,0,0],"f":[null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["filesystemmountpoint",3]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["error",3]],["fserror",4]],[[]],[[]],[[]],[[]],[[],["usize",15]],[[]],[[]],[[]],[[]],null,[[],[["option",4,["fsroot"]],["fsroot",8]]],[[["box",3,["fsroot"]],["fsroot",8]],[["mounterror",4],["result",4,["mounterror"]]]],null,[[],["str",15]],[[]],[[],[["result",4,["fsnode","fserror"]],["fsnode",8],["fserror",4]]],[[],[["result",4,["fsfilehandle","fserror"]],["fsfilehandle",8],["fserror",4]]],null,null,[[],["string",3]],[[],[["str",15],["vec",3,["str"]]]],[[],["u16",15]],[[["usize",15],["option",4,["usize"]]],[["fserror",4],["vec",3,["u8"]],["result",4,["vec","fserror"]]]],[[["usize",15]],[["fserror",4],["result",4,["fserror"]]]],[[],[["fserror",4],["vec",3,["fsnode"]],["result",4,["vec","fserror"]]]],null,[[],["string",3]],[[],["string",3]],[[["str",15]],[["fserror",4],["fsnode",8],["result",4,["fsnode","fserror"]]]],[[],[["fsdirectory",8],["option",4,["fsdirectory"]]]],[[],[["fsfile",8],["option",4,["fsfile"]]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,[[]],[[]],[[]],[[]],null,null,[[],["simplememoryfsfile",3]],[[["simplememoryfs",3]],["bool",15]],[[["simplememoryfsfile",3]],["bool",15]],null,[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[],["usize",15]],[[],["usize",15]],null,[[]],[[]],[[],[["option",4,["fsroot"]],["fsroot",8]]],[[],[["option",4,["fsroot"]],["fsroot",8]]],[[],["str",15]],[[],["str",15]],null,[[["simplememoryfs",3]],["bool",15]],[[["simplememoryfsfile",3]],["bool",15]],[[]],[[],[["result",4,["fsfilehandle","fserror"]],["fsfilehandle",8],["fserror",4]]],null,[[],["u16",15]],[[],["u16",15]],[[["usize",15],["option",4,["usize"]]],[["fserror",4],["vec",3,["u8"]],["result",4,["vec","fserror"]]]],[[["usize",15]],[["fserror",4],["result",4,["fserror"]]]],[[],[["fserror",4],["vec",3,["fsnode"]],["result",4,["vec","fserror"]]]],[[["str",15]],[["fserror",4],["fsnode",8],["result",4,["fsnode","fserror"]]]],[[],[["fsdirectory",8],["option",4,["fsdirectory"]]]],[[],[["fsdirectory",8],["option",4,["fsdirectory"]]]],[[],[["fsfile",8],["option",4,["fsfile"]]]],[[],[["fsfile",8],["option",4,["fsfile"]]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],null,[[],["string",3]],[[["str",15]],["string",3]],[[["str",15]],[["vec",3,["string"]],["string",3]]]],"p":[[4,"FsError"],[4,"MountError"],[3,"FilesystemMountpoint"],[3,"FilesystemBase"],[8,"FsNode"],[8,"FsFile"],[8,"FsFileHandle"],[8,"FsReadDir"],[8,"FsWriteDir"],[13,"StdIoError"],[3,"SimpleMemoryFs"],[3,"SimpleMemoryFsFile"]]},\
"slos_hal":{"doc":"","t":[8,8,8,8,10,10,10,10,11,11,10,10,10,0,10,10,3,3,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["SystemConsole","SystemCpu","SystemHardware","SystemKmainHooks","console","current_cpu","halt","has_requested_return","hook_kmain_loop_head","hook_kmain_loop_inner_part","interrupts_are_enabled","interrupts_disable","interrupts_enable","null_system","system_name","virtualization","NULL_CONSOLE","NullConsole","NullSystem","SYSTEM","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","console","current_cpu","default","deref","deref","eq","fmt","from","from","from","from","halt","has_requested_return","has_requested_return","hook_kmain_loop_head","initialize","initialize","interrupts_are_enabled","interrupts_disable","interrupts_enable","into","into","into","into","is_virtualized","ne","raw_read","raw_write","system_name","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","virtualization"],"q":["slos_hal","","","","","","","","","","","","","","","","slos_hal::null_system","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["System console","System CPU handling","Base system hardware trait","Optional <code>kmain</code> hook methods","Get a reference to the default <code>SystemConsole</code> instance","Current CPU","Halt the current CPU","Has the HAL has requested an immediate kmain return?","<code>kmain</code> loop head hook","<code>kmain</code> inner partial loop hook","Return whether interrupts are enabled on the current CPU","Disable interrupts on the current CPU","Enable interrupts on the current CPU","Placeholder no-op HAL implementation","Name of the crate implementing this system","Virtualization","Global instance of the <code>NullConsole</code>","A <code>SystemConsole</code> implementation that ignores read &amp; write …","A <code>SystemHardware</code> implementation where (almost) everything …","Global instance of the <code>NullSystem</code>","","","","","","","","","","","","","","","","","","","","","","Whether to request a <code>kmain</code> return","","","","","","","","","","","Whether to indicate the system is virtualized","","","","","","","","","","","","","","","","",""],"i":[0,0,0,0,1,1,2,1,3,3,2,2,2,0,1,1,0,0,0,0,4,5,6,7,4,5,6,7,7,7,7,5,6,7,7,4,5,6,7,7,7,7,7,5,6,7,7,7,4,5,6,7,7,7,4,4,7,4,5,6,7,4,5,6,7,4,5,6,7,7],"f":[null,null,null,null,[[],["systemconsole",8]],[[],["systemcpu",8]],[[]],[[],["bool",15]],[[]],[[]],[[],["bool",15]],[[]],[[]],null,[[],["str",15]],[[],["option",4]],null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["systemconsole",8]],[[],["systemcpu",8]],[[]],[[],["unsafecontainer",3]],[[],["unsafecontainer",3]],[[["nullsystem",3]],["bool",15]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[],["bool",15]],null,[[]],[[]],[[]],[[],["bool",15]],[[]],[[]],[[]],[[]],[[]],[[]],null,[[["nullsystem",3]],["bool",15]],[[["usize",15],["option",4,["usize"]]],[["result",4,["vec","fserror"]],["vec",3,["u8"]],["fserror",4]]],[[["usize",15]],[["result",4,["fserror"]],["fserror",4]]],[[],["str",15]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["option",4]]],"p":[[8,"SystemHardware"],[8,"SystemCpu"],[8,"SystemKmainHooks"],[3,"NullConsole"],[3,"NULL_CONSOLE"],[3,"SYSTEM"],[3,"NullSystem"]]},\
"slos_helpers":{"doc":"","t":[18,3,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,14,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11],"n":["MAX_SIZE","StaticCollection","Timer","UnsafeContainer","as_mut_slice","as_slice","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","clone","clone_into","cmp","default","default","deref","deref","eq","fmt","fmt","fmt","fmt","from","from","from","from_iter","function","get","in_milliseconds","increment","increment_ms","into","into","into","into_inner","len","microseconds","new","new","new","partial_cmp","push","replace","seconds","to_owned","to_string","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id"],"q":["slos_helpers","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["Maximum number of entries the collection can hold.","A fixed-size collection.","A time keeper.","Container usable as a static that allows getting a …","Return a mutable slice of the entries within this …","Return a slice of the entries within this collection.","","","","","","","","","","","","","","","","","","","","","","","Get the name of the current function as a <code>&amp;&#39;static str</code>.","Get a mutable reference to the interior value.","Returns this timer’s total time in milliseconds.","Increment the timer by the given number of microseconds.","Increment the timer by the given number of milliseconds.","","","","Consume this container and return the interior value.","Returns the number of elements in the collection.","","Create a new container.","Create a new empty <code>StaticCollection</code>.","Create a new timer with it’s values set to zero.","","Appends an element to the tail end of the collection.","Replace the interior value, returning the old one","","","","","","","","","","","",""],"i":[1,0,0,0,1,1,2,1,3,2,1,3,3,3,3,2,1,2,1,3,2,1,3,3,2,1,3,1,0,2,3,3,3,2,1,3,2,1,3,2,1,3,3,1,2,3,3,3,2,1,3,2,1,3,2,1,3],"f":[null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["timer",3]],[[]],[[["timer",3]],["ordering",4]],[[],["unsafecontainer",3]],[[],["staticcollection",3]],[[]],[[]],[[["timer",3]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[["intoiterator",8]]],null,[[]],[[],["u64",15]],[[["u32",15]]],[[["u32",15]]],[[]],[[]],[[]],[[]],[[],["usize",15]],null,[[],["unsafecontainer",3]],[[]],[[],["timer",3]],[[["timer",3]],[["ordering",4],["option",4,["ordering"]]]],[[]],[[]],null,[[]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]]],"p":[[3,"StaticCollection"],[3,"UnsafeContainer"],[3,"Timer"]]},\
"slos_hosted":{"doc":"","t":[0,0,0,3,3,11,11,11,11,0,11,11,11,11,11,11,11,11,12,11,11,11,11,0,11,11,11,12,11,11,12,11,12,12,11,11,11,11,11,11,11,11,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,13,4,11,11,11,11,11,11,11,11,11,5,5,0,5,5,5,6,3,11,11,0,0,5,11,12,11,11,5,11,11,11,5,5,5,5,5,5,5,5],"n":["hal","host","repl","HostedSystem","SYSTEM","borrow","borrow","borrow_mut","borrow_mut","console","console","current_cpu","default","deref","fmt","from","from","halt","halted","has_requested_return","hook_kmain_loop_head","hook_kmain_loop_inner_part","initialize","interrupts","interrupts_are_enabled","interrupts_disable","interrupts_enable","interrupts_enabled","into","into","kmain_thread","park_if_halted","pending_interrupts","return_next_iter","system_name","try_from","try_from","try_into","try_into","type_id","type_id","virtualization","CONSOLE","Console","borrow","borrow","borrow_mut","borrow_mut","deref","from","from","initialize","into","into","raw_read","raw_write","try_from","try_from","try_into","try_into","type_id","type_id","ClockTick","HostedInterrupt","borrow","borrow_mut","dispatch","fmt","from","into","try_from","try_into","type_id","hosted_kmain","init","interrupts","run_kernel","clock_tick","dispatcher","Callback","Context","borrow","borrow_mut","cmd_basics","cmd_filesystem","default_cmds","from","fs","into","new","run_repl","try_from","try_into","type_id","cmd_echo","cmd_file_read","cmd_file_write_test","cmd_mount_list","cmd_mount_new_memoryfs","cmd_path_join","cmd_path_normalize","cmd_path_split"],"q":["slos_hosted","","","slos_hosted::hal","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","slos_hosted::hal::console","","","","","","","","","","","","","","","","","","","","slos_hosted::hal::interrupts","","","","","","","","","","","slos_hosted::host","","","","slos_hosted::host::interrupts","","slos_hosted::repl","","","","","","","","","","","","","","","slos_hosted::repl::cmd_basics","slos_hosted::repl::cmd_filesystem","","","","","",""],"d":["Hosted “hardware” abstraction layer implementation","Hosted slOS kernel","Debugging REPL for parts of slOS","Hosted “hardware” abstraction layer implementation","Global instance of the <code>HostedSystem</code>","","","","","Forwarding to host stdin/stdout","","","","","","","","","Whether the hosted machine is halted until the next …","","","","","Virtual interrupts","","","","Whether interrupts are enabled","","","kmain <code>std::thread::Thread</code>","If halted, and current thread is kmain, park thread","Queue of pending interrupts","Whether to make kmain return in its next iteration","","","","","","","","","Global <code>Console</code> instance","Hosted <code>SystemConsole</code> implementation","","","","","","","","","","","","","","","","","","","Clock tick interrupt","A virtual interrupt","","","Perform handling for <code>self</code> as an interrupt","","","","","","","Run <code>slos::kmain</code> in our hosted environment","Initialize the environment for the hosted kernel","Virtual interrupt handlers","Start the hosted kernel","Update the system clock and queue tick interrupts","Dispatch virtual interrupts, if enabled","REPL command callback type, where <code>T</code> is the command context","Default command context type","","","Basic command set","Filesystem command set","Default commands, using <code>Context</code> as the command context","","Filesystem","","","Run the REPL with the given command list","","","","Echo the given arguments back to the console","Read the contents of the given file","Write the string <code>&quot;hello world!&quot;</code> to the given file","Print the list of currently mounted filesystems","Mount a new <code>SimpleMemoryFs</code> on the given path","Print a joined path","Print a normalized path","Debug print a split path"],"i":[0,0,0,0,0,1,2,1,2,0,2,2,2,1,2,1,2,2,2,2,2,2,1,0,2,2,2,2,1,2,2,2,2,2,2,1,2,1,2,1,2,2,0,0,3,4,3,4,4,3,4,4,3,4,3,3,3,4,3,4,3,4,5,0,5,5,5,5,5,5,5,5,5,0,0,0,0,0,0,0,0,6,6,0,0,0,6,6,6,6,0,6,6,6,0,0,0,0,0,0,0,0],"f":[null,null,null,null,null,[[]],[[]],[[]],[[]],null,[[],["systemconsole",8]],[[],["systemcpu",8]],[[]],[[],["unsafecontainer",3]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],null,[[],["bool",15]],[[]],[[]],[[]],null,[[],["bool",15]],[[]],[[]],null,[[]],[[]],null,[[]],null,null,[[],["str",15]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["option",4]],null,null,[[]],[[]],[[]],[[]],[[],["unsafecontainer",3]],[[]],[[]],[[]],[[]],[[]],[[["usize",15],["option",4,["usize"]]],[["result",4,["vec","fserror"]],["vec",3,["u8"]],["fserror",4]]],[[["usize",15]],[["result",4,["fserror"]],["fserror",4]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],null,null,[[]],[[]],[[]],[[["formatter",3]],["result",6]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["result",6]],[[["option",4,["pathbuf"]],["pathbuf",3],["string",3]],["result",6]],null,[[],["result",6]],[[]],[[["thread",3]]],null,null,[[]],[[]],null,null,[[],[["hashmap",3,["string","hashmap"]],["hashmap",3,["string","box"]],["string",3]]],[[]],null,[[]],[[]],[[["hashmap",3,["string","hashmap"]],["hashmap",3,["string","box"]],["string",3]],["result",6]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[["context",3]],["result",6]],[[["context",3]],["result",6]],[[["context",3]],["result",6]],[[["context",3]],["result",6]],[[["context",3]],["result",6]],[[["context",3]],["result",6]],[[["context",3]],["result",6]],[[["context",3]],["result",6]]],"p":[[3,"SYSTEM"],[3,"HostedSystem"],[3,"Console"],[3,"CONSOLE"],[4,"HostedInterrupt"],[3,"Context"]]},\
"slos_log":{"doc":"","t":[14,14,14,14,14,14],"n":["debug","error","info","log","trace","warn"],"q":["slos_log","","","","",""],"d":["Log a debug message","Log an error message","Log an info message","Log a message with a given level","Log a trace message","Log a warning message"],"i":[0,0,0,0,0,0],"f":[null,null,null,null,null,null],"p":[]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};