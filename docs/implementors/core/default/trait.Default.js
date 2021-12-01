(function() {var implementors = {};
implementors["mmtk"] = [{"text":"impl&lt;VM:&nbsp;<a class=\"trait\" href=\"mmtk/vm/trait.VMBinding.html\" title=\"trait mmtk::vm::VMBinding\">VMBinding</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"mmtk/mmtk/struct.MMTK.html\" title=\"struct mmtk::mmtk::MMTK\">MMTK</a>&lt;VM&gt;","synthetic":false,"types":["mmtk::mmtk::MMTK"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"mmtk/policy/space/struct.SFTMap.html\" title=\"struct mmtk::policy::space::SFTMap\">SFTMap</a>&lt;'a&gt;","synthetic":false,"types":["mmtk::policy::space::SFTMap"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"mmtk/policy/immix/block/struct.BlockList.html\" title=\"struct mmtk::policy::immix::block::BlockList\">BlockList</a>","synthetic":false,"types":["mmtk::policy::immix::block::BlockList"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"mmtk/policy/immix/defrag/struct.Defrag.html\" title=\"struct mmtk::policy::immix::defrag::Defrag\">Defrag</a>","synthetic":false,"types":["mmtk::policy::immix::defrag::Defrag"]},{"text":"impl&lt;VM:&nbsp;<a class=\"trait\" href=\"mmtk/vm/trait.VMBinding.html\" title=\"trait mmtk::vm::VMBinding\">VMBinding</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"mmtk/plan/controller_collector_context/struct.ControllerCollectorContext.html\" title=\"struct mmtk::plan::controller_collector_context::ControllerCollectorContext\">ControllerCollectorContext</a>&lt;VM&gt;","synthetic":false,"types":["mmtk::plan::controller_collector_context::ControllerCollectorContext"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"mmtk/plan/mutator_context/struct.ReservedAllocators.html\" title=\"struct mmtk::plan::mutator_context::ReservedAllocators\">ReservedAllocators</a>","synthetic":false,"types":["mmtk::plan::mutator_context::ReservedAllocators"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"mmtk/scheduler/stat/struct.SchedulerStat.html\" title=\"struct mmtk::scheduler::stat::SchedulerStat\">SchedulerStat</a>","synthetic":false,"types":["mmtk::scheduler::stat::SchedulerStat"]},{"text":"impl&lt;C&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"mmtk/scheduler/stat/struct.WorkerLocalStat.html\" title=\"struct mmtk::scheduler::stat::WorkerLocalStat\">WorkerLocalStat</a>&lt;C&gt;","synthetic":false,"types":["mmtk::scheduler::stat::WorkerLocalStat"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"mmtk/scheduler/work_counter/struct.WorkCounterBase.html\" title=\"struct mmtk::scheduler::work_counter::WorkCounterBase\">WorkCounterBase</a>","synthetic":false,"types":["mmtk::scheduler::work_counter::WorkCounterBase"]},{"text":"impl&lt;W:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> + <a class=\"trait\" href=\"mmtk/plan/global/trait.CopyContext.html\" title=\"trait mmtk::plan::global::CopyContext\">CopyContext</a> + <a class=\"trait\" href=\"mmtk/scheduler/worker/trait.GCWorkerLocal.html\" title=\"trait mmtk::scheduler::worker::GCWorkerLocal\">GCWorkerLocal</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"mmtk/scheduler/gc_work/struct.PrepareCollector.html\" title=\"struct mmtk::scheduler::gc_work::PrepareCollector\">PrepareCollector</a>&lt;W&gt;","synthetic":false,"types":["mmtk::scheduler::gc_work::PrepareCollector"]},{"text":"impl&lt;W:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> + <a class=\"trait\" href=\"mmtk/plan/global/trait.CopyContext.html\" title=\"trait mmtk::plan::global::CopyContext\">CopyContext</a> + <a class=\"trait\" href=\"mmtk/scheduler/worker/trait.GCWorkerLocal.html\" title=\"trait mmtk::scheduler::worker::GCWorkerLocal\">GCWorkerLocal</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"mmtk/scheduler/gc_work/struct.ReleaseCollector.html\" title=\"struct mmtk::scheduler::gc_work::ReleaseCollector\">ReleaseCollector</a>&lt;W&gt;","synthetic":false,"types":["mmtk::scheduler::gc_work::ReleaseCollector"]},{"text":"impl&lt;ScanEdges:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> + <a class=\"trait\" href=\"mmtk/scheduler/gc_work/trait.ProcessEdgesWork.html\" title=\"trait mmtk::scheduler::gc_work::ProcessEdgesWork\">ProcessEdgesWork</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"mmtk/scheduler/gc_work/struct.StopMutators.html\" title=\"struct mmtk::scheduler::gc_work::StopMutators\">StopMutators</a>&lt;ScanEdges&gt;","synthetic":false,"types":["mmtk::scheduler::gc_work::StopMutators"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"mmtk/scheduler/gc_work/struct.EndOfGC.html\" title=\"struct mmtk::scheduler::gc_work::EndOfGC\">EndOfGC</a>","synthetic":false,"types":["mmtk::scheduler::gc_work::EndOfGC"]},{"text":"impl&lt;E:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> + <a class=\"trait\" href=\"mmtk/scheduler/gc_work/trait.ProcessEdgesWork.html\" title=\"trait mmtk::scheduler::gc_work::ProcessEdgesWork\">ProcessEdgesWork</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"mmtk/scheduler/gc_work/struct.ProcessWeakRefs.html\" title=\"struct mmtk::scheduler::gc_work::ProcessWeakRefs\">ProcessWeakRefs</a>&lt;E&gt;","synthetic":false,"types":["mmtk::scheduler::gc_work::ProcessWeakRefs"]},{"text":"impl&lt;Edges:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> + <a class=\"trait\" href=\"mmtk/scheduler/gc_work/trait.ProcessEdgesWork.html\" title=\"trait mmtk::scheduler::gc_work::ProcessEdgesWork\">ProcessEdgesWork</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"mmtk/scheduler/gc_work/struct.ScanStackRoots.html\" title=\"struct mmtk::scheduler::gc_work::ScanStackRoots\">ScanStackRoots</a>&lt;Edges&gt;","synthetic":false,"types":["mmtk::scheduler::gc_work::ScanStackRoots"]},{"text":"impl&lt;Edges:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> + <a class=\"trait\" href=\"mmtk/scheduler/gc_work/trait.ProcessEdgesWork.html\" title=\"trait mmtk::scheduler::gc_work::ProcessEdgesWork\">ProcessEdgesWork</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"mmtk/scheduler/gc_work/struct.ScanVMSpecificRoots.html\" title=\"struct mmtk::scheduler::gc_work::ScanVMSpecificRoots\">ScanVMSpecificRoots</a>&lt;Edges&gt;","synthetic":false,"types":["mmtk::scheduler::gc_work::ScanVMSpecificRoots"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"enum\" href=\"mmtk/util/alloc/allocators/enum.AllocatorSelector.html\" title=\"enum mmtk::util::alloc::allocators::AllocatorSelector\">AllocatorSelector</a>","synthetic":false,"types":["mmtk::util::alloc::allocators::AllocatorSelector"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"mmtk/util/opaque_pointer/struct.OpaquePointer.html\" title=\"struct mmtk::util::opaque_pointer::OpaquePointer\">OpaquePointer</a>","synthetic":false,"types":["mmtk::util::opaque_pointer::OpaquePointer"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"mmtk/util/reference_processor/struct.ReferenceProcessors.html\" title=\"struct mmtk::util::reference_processor::ReferenceProcessors\">ReferenceProcessors</a>","synthetic":false,"types":["mmtk::util::reference_processor::ReferenceProcessors"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"mmtk/util/analysis/obj_size/struct.PerSizeClassObjectCounter.html\" title=\"struct mmtk::util::analysis::obj_size::PerSizeClassObjectCounter\">PerSizeClassObjectCounter</a>","synthetic":false,"types":["mmtk::util::analysis::obj_size::PerSizeClassObjectCounter"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"mmtk/util/analysis/struct.GcHookWork.html\" title=\"struct mmtk::util::analysis::GcHookWork\">GcHookWork</a>","synthetic":false,"types":["mmtk::util::analysis::GcHookWork"]},{"text":"impl&lt;VM:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> + <a class=\"trait\" href=\"mmtk/vm/trait.VMBinding.html\" title=\"trait mmtk::vm::VMBinding\">VMBinding</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"mmtk/util/analysis/struct.AnalysisManager.html\" title=\"struct mmtk::util::analysis::AnalysisManager\">AnalysisManager</a>&lt;VM&gt;","synthetic":false,"types":["mmtk::util::analysis::AnalysisManager"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"mmtk/util/finalizable_processor/struct.FinalizableProcessor.html\" title=\"struct mmtk::util::finalizable_processor::FinalizableProcessor\">FinalizableProcessor</a>","synthetic":false,"types":["mmtk::util::finalizable_processor::FinalizableProcessor"]},{"text":"impl&lt;E:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> + <a class=\"trait\" href=\"mmtk/scheduler/gc_work/trait.ProcessEdgesWork.html\" title=\"trait mmtk::scheduler::gc_work::ProcessEdgesWork\">ProcessEdgesWork</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"mmtk/util/finalizable_processor/struct.Finalization.html\" title=\"struct mmtk::util::finalizable_processor::Finalization\">Finalization</a>&lt;E&gt;","synthetic":false,"types":["mmtk::util::finalizable_processor::Finalization"]},{"text":"impl&lt;E:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> + <a class=\"trait\" href=\"mmtk/scheduler/gc_work/trait.ProcessEdgesWork.html\" title=\"trait mmtk::scheduler::gc_work::ProcessEdgesWork\">ProcessEdgesWork</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"mmtk/util/finalizable_processor/struct.ForwardFinalization.html\" title=\"struct mmtk::util::finalizable_processor::ForwardFinalization\">ForwardFinalization</a>&lt;E&gt;","synthetic":false,"types":["mmtk::util::finalizable_processor::ForwardFinalization"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"mmtk/util/heap/accounting/struct.PageAccounting.html\" title=\"struct mmtk::util::heap::accounting::PageAccounting\">PageAccounting</a>","synthetic":false,"types":["mmtk::util::heap::accounting::PageAccounting"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"mmtk/util/heap/layout/fragmented_mapper/struct.FragmentedMapper.html\" title=\"struct mmtk::util::heap::layout::fragmented_mapper::FragmentedMapper\">FragmentedMapper</a>","synthetic":false,"types":["mmtk::util::heap::layout::fragmented_mapper::FragmentedMapper"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"mmtk/util/heap/layout/map64/struct.Map64.html\" title=\"struct mmtk::util::heap::layout::map64::Map64\">Map64</a>","synthetic":false,"types":["mmtk::util::heap::layout::map64::Map64"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"mmtk/util/metadata/side_metadata/sanity/struct.SideMetadataSanity.html\" title=\"struct mmtk::util::metadata::side_metadata::sanity::SideMetadataSanity\">SideMetadataSanity</a>","synthetic":false,"types":["mmtk::util::metadata::side_metadata::sanity::SideMetadataSanity"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"mmtk/util/options/struct.Options.html\" title=\"struct mmtk::util::options::Options\">Options</a>","synthetic":false,"types":["mmtk::util::options::Options"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"mmtk/util/sanity/sanity_checker/struct.SanityChecker.html\" title=\"struct mmtk::util::sanity::sanity_checker::SanityChecker\">SanityChecker</a>","synthetic":false,"types":["mmtk::util::sanity::sanity_checker::SanityChecker"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"mmtk/util/treadmill/struct.TreadMill.html\" title=\"struct mmtk::util::treadmill::TreadMill\">TreadMill</a>","synthetic":false,"types":["mmtk::util::treadmill::TreadMill"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()