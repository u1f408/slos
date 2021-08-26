(function() {var implementors = {};
implementors["slos"] = [{"text":"impl Freeze for <a class=\"enum\" href=\"slos/enum.KernelError.html\" title=\"enum slos::KernelError\">KernelError</a>","synthetic":true,"types":["slos::errors::KernelError"]},{"text":"impl Freeze for <a class=\"struct\" href=\"slos/clock/struct.BOOT_CLOCK.html\" title=\"struct slos::clock::BOOT_CLOCK\">BOOT_CLOCK</a>","synthetic":true,"types":["slos::clock::BOOT_CLOCK"]},{"text":"impl !Freeze for <a class=\"struct\" href=\"slos/filesystem/devices/struct.SystemDeviceCollection.html\" title=\"struct slos::filesystem::devices::SystemDeviceCollection\">SystemDeviceCollection</a>","synthetic":true,"types":["slos::filesystem::devices::SystemDeviceCollection"]},{"text":"impl !Freeze for <a class=\"struct\" href=\"slos/filesystem/devices/struct.SystemDeviceFile.html\" title=\"struct slos::filesystem::devices::SystemDeviceFile\">SystemDeviceFile</a>","synthetic":true,"types":["slos::filesystem::devices::SystemDeviceFile"]},{"text":"impl Freeze for <a class=\"struct\" href=\"slos/filesystem/struct.FSBASE.html\" title=\"struct slos::filesystem::FSBASE\">FSBASE</a>","synthetic":true,"types":["slos::filesystem::FSBASE"]},{"text":"impl Freeze for <a class=\"struct\" href=\"slos/struct.KMAIN_INIT_PARTIALS.html\" title=\"struct slos::KMAIN_INIT_PARTIALS\">KMAIN_INIT_PARTIALS</a>","synthetic":true,"types":["slos::KMAIN_INIT_PARTIALS"]},{"text":"impl Freeze for <a class=\"struct\" href=\"slos/struct.KMAIN_LOOP_PARTIALS.html\" title=\"struct slos::KMAIN_LOOP_PARTIALS\">KMAIN_LOOP_PARTIALS</a>","synthetic":true,"types":["slos::KMAIN_LOOP_PARTIALS"]}];
implementors["slos_filesystem"] = [{"text":"impl Freeze for <a class=\"enum\" href=\"slos_filesystem/enum.FsError.html\" title=\"enum slos_filesystem::FsError\">FsError</a>","synthetic":true,"types":["slos_filesystem::errors::FsError"]},{"text":"impl Freeze for <a class=\"enum\" href=\"slos_filesystem/enum.MountError.html\" title=\"enum slos_filesystem::MountError\">MountError</a>","synthetic":true,"types":["slos_filesystem::errors::MountError"]},{"text":"impl Freeze for <a class=\"struct\" href=\"slos_filesystem/memory/struct.SimpleMemoryFs.html\" title=\"struct slos_filesystem::memory::SimpleMemoryFs\">SimpleMemoryFs</a>","synthetic":true,"types":["slos_filesystem::memory::SimpleMemoryFs"]},{"text":"impl Freeze for <a class=\"struct\" href=\"slos_filesystem/memory/struct.SimpleMemoryFsFile.html\" title=\"struct slos_filesystem::memory::SimpleMemoryFsFile\">SimpleMemoryFsFile</a>","synthetic":true,"types":["slos_filesystem::memory::SimpleMemoryFsFile"]},{"text":"impl !Freeze for <a class=\"struct\" href=\"slos_filesystem/struct.FilesystemMountpoint.html\" title=\"struct slos_filesystem::FilesystemMountpoint\">FilesystemMountpoint</a>","synthetic":true,"types":["slos_filesystem::FilesystemMountpoint"]},{"text":"impl !Freeze for <a class=\"struct\" href=\"slos_filesystem/struct.FilesystemBase.html\" title=\"struct slos_filesystem::FilesystemBase\">FilesystemBase</a>","synthetic":true,"types":["slos_filesystem::FilesystemBase"]}];
implementors["slos_hal"] = [{"text":"impl Freeze for <a class=\"struct\" href=\"slos_hal/null_console/struct.NULL_CONSOLE.html\" title=\"struct slos_hal::null_console::NULL_CONSOLE\">NULL_CONSOLE</a>","synthetic":true,"types":["slos_hal::null_console::NULL_CONSOLE"]},{"text":"impl Freeze for <a class=\"struct\" href=\"slos_hal/null_console/struct.NullConsole.html\" title=\"struct slos_hal::null_console::NullConsole\">NullConsole</a>","synthetic":true,"types":["slos_hal::null_console::NullConsole"]}];
implementors["slos_helpers"] = [{"text":"impl Freeze for <a class=\"struct\" href=\"slos_helpers/struct.Timer.html\" title=\"struct slos_helpers::Timer\">Timer</a>","synthetic":true,"types":["slos_helpers::timer::Timer"]},{"text":"impl&lt;T&gt; !Freeze for <a class=\"struct\" href=\"slos_helpers/struct.UnsafeContainer.html\" title=\"struct slos_helpers::UnsafeContainer\">UnsafeContainer</a>&lt;T&gt;","synthetic":true,"types":["slos_helpers::unsafecontainer::UnsafeContainer"]},{"text":"impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"slos_helpers/struct.StaticCollection.html\" title=\"struct slos_helpers::StaticCollection\">StaticCollection</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Freeze,&nbsp;</span>","synthetic":true,"types":["slos_helpers::staticcollection::StaticCollection"]}];
implementors["slos_hosted"] = [{"text":"impl Freeze for <a class=\"struct\" href=\"slos_hosted/hal/console/struct.CONSOLE.html\" title=\"struct slos_hosted::hal::console::CONSOLE\">CONSOLE</a>","synthetic":true,"types":["slos_hosted::hal::console::CONSOLE"]},{"text":"impl Freeze for <a class=\"struct\" href=\"slos_hosted/hal/console/struct.Console.html\" title=\"struct slos_hosted::hal::console::Console\">Console</a>","synthetic":true,"types":["slos_hosted::hal::console::Console"]},{"text":"impl Freeze for <a class=\"enum\" href=\"slos_hosted/hal/interrupts/enum.HostedInterrupt.html\" title=\"enum slos_hosted::hal::interrupts::HostedInterrupt\">HostedInterrupt</a>","synthetic":true,"types":["slos_hosted::hal::interrupts::HostedInterrupt"]},{"text":"impl Freeze for <a class=\"struct\" href=\"slos_hosted/hal/struct.SYSTEM.html\" title=\"struct slos_hosted::hal::SYSTEM\">SYSTEM</a>","synthetic":true,"types":["slos_hosted::hal::SYSTEM"]},{"text":"impl Freeze for <a class=\"struct\" href=\"slos_hosted/hal/struct.HostedSystem.html\" title=\"struct slos_hosted::hal::HostedSystem\">HostedSystem</a>","synthetic":true,"types":["slos_hosted::hal::HostedSystem"]},{"text":"impl !Freeze for <a class=\"struct\" href=\"slos_hosted/repl/struct.Context.html\" title=\"struct slos_hosted::repl::Context\">Context</a>","synthetic":true,"types":["slos_hosted::repl::context::Context"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()