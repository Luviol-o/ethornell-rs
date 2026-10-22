use crate::{Value, VmError, VmResult};
use ethornell_script::{candidate_call_name, native_abi};

/// Canonical identifier for a native BGI dispatch entry.
///
/// `group` selects a dispatch table (for example `0x80` for system and `0x90`
/// for graph calls) and `id` selects an entry in that table. Runtime code must
/// pass this value as one unit instead of carrying unrelated `group` and `id`
/// integers through every layer.
pub use ethornell_script::calls::CallKey as NativeOpcode;

/// Canonical constants for opcodes whose semantics have been recovered well
/// enough to have typed wrappers. New wrappers must add their constant here so
/// runtime dispatch, tests, and documentation all refer to the same key.
pub mod opcodes {
    use super::NativeOpcode;

    /// Seeds the target Microsoft CRT `rand` state.
    pub const SYS_SRAND: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x00,
    };
    /// Returns one Microsoft CRT `rand` value in the inclusive range 0..=0x7FFF.
    pub const SYS_RAND: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x01,
    };
    /// Builds a wider random value from three CRT `rand` calls and reduces it modulo max.
    pub const SYS_RAND_MAX: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x02,
    };
    /// Returns the target engine millisecond clock.
    pub const SYS_GET_ENGINE_TICK: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x04,
    };
    /// Writes a nanosecond-scaled high-resolution counter and returns success.
    pub const SYS_QUERY_PERFORMANCE_COUNTER_NS: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x05,
    };
    /// Enables/disables and resets the target performance sampler.
    pub const SYS_SET_PERFORMANCE_PROFILING: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x06,
    };
    /// Writes one target performance metric selected by an integer selector.
    pub const SYS_READ_PERFORMANCE_METRIC: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x07,
    };
    /// Pushes the target client-space cursor X and Y coordinates.
    pub const SYS_READ_CURSOR_POINT: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x08,
    };
    /// Returns the renderer's most recent presentation timestamp/status value.
    pub const SYS_QUERY_PRESENTATION_STATE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x09,
    };
    /// Copies the 16-DWORD target graphics capability cache.
    pub const SYS_COPY_GRAPHICS_CAPABILITIES: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x0A,
    };
    /// Returns the renderer-owned memory/capacity field at native object offset +0x58.
    pub const SYS_QUERY_GRAPHICS_MEMORY_METRIC: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x0B,
    };
    /// Writes a Win32 SYSTEMTIME-compatible local-time record.
    pub const SYS_GET_LOCAL_TIME: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x0C,
    };
    /// Pushes clamped total and available physical memory byte counts.
    pub const SYS_GET_PHYSICAL_MEMORY: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x0D,
    };
    /// Returns the target minimize/restore latch maintained by the window procedure.
    pub const SYS_QUERY_WINDOW_MINIMIZE_LATCH: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x0E,
    };
    /// Returns whether the target window is active.
    pub const SYS_QUERY_WINDOW_ACTIVE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x0F,
    };
    /// Stores the input enable/mapping gate and clears target per-key state records.
    pub const SYS_RESET_INPUT_CONFIGURATION: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x10,
    };
    /// Returns the high-bit down state of one target logical input descriptor.
    pub const SYS_QUERY_KEY_DOWN: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x11,
    };
    /// Sums the current count/state field for a zero-terminated descriptor list.
    pub const SYS_SUM_INPUT_DESCRIPTOR_STATE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x12,
    };
    /// Selects normal or swapped logical mouse-button mapping; accepts only 0 or 1.
    pub const SYS_SET_MOUSE_BUTTON_MAPPING_MODE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x1E,
    };

    /// Allocates one target VM memory-class block and returns its opaque BP pointer.
    pub const SYS_ALLOC: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x20,
    };
    /// Releases one target VM memory-class block; a null pointer succeeds.
    pub const SYS_FREE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x21,
    };
    /// Counts files matched by one Win32 pattern, optionally recursing into directories.
    pub const SYS_COUNT_FILES: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x24,
    };
    /// Enumerates matched files into a packed sequence of NUL-terminated names.
    pub const SYS_ENUMERATE_FILES: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x25,
    };
    /// Enumerates immediate subdirectory names into a packed NUL-terminated buffer.
    pub const SYS_ENUMERATE_DIRECTORIES: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x26,
    };
    /// Calls Win32 MoveFileA with the target's destination-first BP argument order.
    pub const SYS_MOVE_FILE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x27,
    };
    /// Calls Win32 CreateDirectoryA for exactly one directory level.
    pub const SYS_CREATE_DIRECTORY: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x28,
    };
    /// Calls Win32 RemoveDirectoryA.
    pub const SYS_REMOVE_DIRECTORY: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x29,
    };
    /// Returns whether GetFileAttributesA reports a directory.
    pub const SYS_DIRECTORY_EXISTS: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x2A,
    };
    /// Splits a multibyte path into drive, directory, filename and extension outputs.
    pub const SYS_SPLIT_PATH: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x2B,
    };
    /// Returns the raw Win32 GetFileAttributesA value.
    pub const SYS_GET_FILE_ATTRIBUTES: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x2C,
    };
    /// Calls Win32 SetFileAttributesA.
    pub const SYS_SET_FILE_ATTRIBUTES: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x2D,
    };
    /// Clears destination read-only state and calls CopyFileA with overwrite enabled.
    pub const SYS_COPY_FILE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x2F,
    };
    /// Reads one resource/search-root file into a BP buffer and returns its byte size.
    pub const SYS_READ_FILE_BYTES: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x30,
    };
    /// Reads a whole file or an explicit byte range and returns a target status code.
    pub const SYS_READ_FILE_RANGE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x31,
    };
    /// Writes exactly length bytes and returns whether the full count was written.
    pub const SYS_WRITE_FILE_BYTES: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x32,
    };
    /// Deletes root\file, or the primary search-root file when root is null.
    pub const SYS_DELETE_FILE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x33,
    };
    /// Tests a file against the target primary and secondary resource roots.
    pub const SYS_FILE_EXISTS: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x34,
    };
    /// Returns a file/resource size from the target search roots.
    pub const SYS_FILE_SIZE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x35,
    };
    /// Enables or disables additional linked-list resource search paths.
    pub const SYS_SET_ADDITIONAL_RESOURCE_SEARCH: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x36,
    };
    /// Prepends one directory to the target additional resource path list.
    pub const SYS_PREPEND_RESOURCE_SEARCH_PATH: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x37,
    };
    /// Registers a named composite archive from a zero-terminated component list.
    pub const SYS_REGISTER_COMPOSITE_ARCHIVE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x38,
    };
    /// Validates and stores the auxiliary filesystem root used by native file operations.
    pub const SYS_SET_VALIDATED_FILE_ROOT: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x39,
    };
    /// Copies one Win32 special-folder path selected by mode 0..=5.
    pub const SYS_GET_SPECIAL_FOLDER: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x3A,
    };
    /// Opens the target Win32 open/save file dialog.
    pub const SYS_OPEN_FILE_DIALOG: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x3B,
    };
    /// Verifies a required resource file, prompting retry/quit on the target host.
    pub const SYS_REQUIRE_RESOURCE_FILE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x3C,
    };
    /// Copies primary or secondary configured root text to a caller buffer.
    pub const SYS_GET_CONFIGURED_ROOT: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x3D,
    };
    /// If path names a directory, stores path plus a trailing backslash as the primary root.
    pub const SYS_SET_PRIMARY_ROOT: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x3E,
    };
    /// Configures and locates a removable-media archive, storing its root on success.
    pub const SYS_CONFIGURE_REMOVABLE_ARCHIVE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x3F,
    };
    /// Appends one BP module to the current CThread code region and returns its code base.
    pub const SYS_LOAD_PROGRAM_MODULE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x40,
    };
    /// Removes the most recently appended BP module and returns the remaining module count.
    pub const SYS_FREE_LAST_PROGRAM_MODULE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x41,
    };
    /// Creates a child CThread, loads one BP module and returns the integer thread identifier.
    pub const SYS_LOAD_PROGRAM_THREAD: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x44,
    };
    /// Release-build table entry whose ABI exposes no value or side effect to BP code.
    pub const SYS_RTC_NOOP: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x45,
    };
    /// Returns CThread+8, the current integer scheduler/thread identifier.
    pub const SYS_CURRENT_THREAD_ID: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x46,
    };
    /// Recursively searches the CThread tree for a nonzero integer thread identifier.
    pub const SYS_THREAD_EXISTS: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x47,
    };

    /// Returns the monotonically increasing target window/input-message serial.
    pub const SYS_INPUT_MESSAGE_SERIAL: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x13,
    };
    /// Writes target global dword_506A48, the final configured-input enable gate.
    pub const SYS_SET_INPUT_MASTER_GATE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x14,
    };
    /// Writes target global dword_566828, the configured-input latched state.
    pub const SYS_SET_INPUT_LATCHED_STATE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x15,
    };
    pub const SYS_SAMPLE_CONFIGURED_INPUT: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x16,
    };
    pub const SYS_QUERY_CONFIGURED_INPUT_GATE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x17,
    };
    /// Registers `(scope << 16) | 0xFFFF` in both target input-scope lists.
    pub const SYS_REGISTER_INPUT_SCOPE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x18,
    };
    /// Queries and then removes `(scope << 16) | 0xFFFF` from both lists.
    pub const SYS_QUERY_AND_UNREGISTER_INPUT_SCOPE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x19,
    };
    pub const SYS_QUERY_INPUT_EVENT_BITS: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x1A,
    };
    /// Registers a zero-terminated descriptor list for one target input class.
    pub const SYS_REGISTER_INPUT_CLASS_DESCRIPTORS: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x1B,
    };
    pub const SYS_QUERY_INPUT_CLASS_LEVEL: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x1C,
    };
    /// Validates one packed input scope against the pointer or keyboard scope
    /// registry, then drains the selected logical input's target event counter.
    pub const SYS_QUERY_SCOPED_INPUT_EVENT: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x1D,
    };
    /// Starts the target global cursor-motion interpolator.
    pub const SYS_CONFIGURE_CURSOR_MOTION: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x1F,
    };
    /// Writes dword_507690, the auxiliary input mask shared by CProcedure
    /// and CProcDspMsg filtering.
    pub const SYS_SET_MESSAGE_AUXILIARY_INPUT_MASK: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x1F,
    };
    pub const SYS_ENQUEUE_MESSAGE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x48,
    };
    pub const SYS_DEQUEUE_MESSAGE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x49,
    };
    pub const SYS_ENQUEUE_MESSAGE_ARRAY: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x4A,
    };
    pub const SYS_DEQUEUE_MESSAGE_ARRAY: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x4B,
    };
    pub const SYS_INVOKE_THREAD_CALLBACK: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x4C,
    };
    /// Stores the process-global system wait-state value at dword_507688.
    pub const SYS_SET_SYSTEM_WAIT_STATE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x50,
    };
    /// Pops one Win32 message identifier, allocates a 0x24-byte
    /// `CProcWaitWndMsg`, registers a waiter node, and suspends the thread.
    pub const SYS_WAIT_WINDOW_MESSAGE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x54,
    };
    /// Sets `CThread+0x84` to `current_tick + duration_ms`.
    pub const SYS_SET_THREAD_TIMER: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x58,
    };
    /// Sets CThread+0x84 and immediately returns whether the remaining time is nonzero.
    pub const SYS_SET_AND_QUERY_THREAD_TIMER: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x59,
    };
    /// Reads remaining time from `CThread+0x84`. When positive, allocates a
    /// 0x20-byte `CProcWaitTiming`; always pushes whether a wait was installed.
    pub const SYS_WAIT_THREAD_TIMER: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x5A,
    };
    // Backward-compatible internal aliases retained for older tests/tools.
    pub const SYS_PROCEDURE_54: NativeOpcode = SYS_WAIT_WINDOW_MESSAGE;
    pub const SYS_PROCEDURE_5A: NativeOpcode = SYS_WAIT_THREAD_TIMER;
    pub const SYS_WAIT_TIMING_EX: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x5C,
    };
    pub const SYS_SWITCH_PROGRAM: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x5E,
    };
    /// Target sub_4892D0 returns native interpreter status 1 without touching
    /// the BP stack or CThread state. The outer interpreter resumes at the
    /// instruction following this call on its next cooperative pass.
    pub const SYS_YIELD: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x5F,
    };
    /// Stores the target main-loop wait/pacing override in dword_503EF8.
    pub const SYS_SET_MAIN_LOOP_WAIT_OVERRIDE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x52,
    };
    /// Arms exactly the next binary-read or BMV-decode call for its procedure path.
    pub const SYS_ARM_NEXT_BINARY_ASYNC: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x53,
    };
    /// Pins cooperative scheduling to the current CThread while nonzero.
    pub const SYS_SET_EXCLUSIVE_THREAD: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x5D,
    };
    pub const SYS_CONFIGURE_DISPLAY_MODE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x60,
    };
    pub const SYS_QUERY_FULLSCREEN: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x61,
    };
    pub const SYS_CONFIGURE_FULLSCREEN_HOTKEYS: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x62,
    };
    pub const SYS_SET_ASPECT_PRESERVING_SCALING: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x63,
    };
    pub const SYS_SET_WINDOW_VISIBLE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x64,
    };
    pub const SYS_MINIMIZE_WINDOW: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x65,
    };
    pub const SYS_SET_WINDOW_TITLE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x66,
    };
    pub const SYS_SET_CURSOR_INDEX: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x67,
    };
    pub const SYS_SET_NATIVE_CLOSE_MODE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x68,
    };
    pub const SYS_REQUEST_WINDOW_CLOSE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x69,
    };
    /// Native status 6 terminates the interpreter, including the active async task.
    pub const SYS_TERMINATE_INTERPRETER: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x6A,
    };
    pub const SYS_SELECT_BOOTSTRAP: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x6B,
    };
    pub const SYS_SET_FILE_DROP_ENABLED: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x6C,
    };
    pub const SYS_QUERY_DROPPED_FILE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x6D,
    };
    pub const SYS_SET_RASTER_WAIT_ENABLED: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x6E,
    };
    pub const SYS_QUERY_DISPLAY_ASPECT_MISMATCH: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x6F,
    };
    pub const SYS_ALLOCATE_GLOBAL_CONFIG: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x70,
    };
    pub const SYS_CLEAR_GLOBAL_CONFIG: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x71,
    };
    pub const SYS_SET_SAVE_DATA_INTEGRITY: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x74,
    };
    pub const SYS_SAVE_CONFIG_SLOT: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x78,
    };
    pub const SYS_LOAD_CONFIG_SLOT: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x79,
    };
    pub const SYS_READ_CONFIG_SLOT_HEADER: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x7A,
    };
    pub const SYS_VALIDATE_CONFIG_SLOT: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x7B,
    };
    // Compatibility alias for older internal code and external audit scripts.
    pub const SYS_END_SCHEDULER_PASS: NativeOpcode = SYS_TERMINATE_INTERPRETER;

    pub const SYS_INTERN_RESOURCE_NAME: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x84,
    };
    pub const SYS_RESOURCE_NAME_EXISTS: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x85,
    };
    pub const SYS_CREATE_OR_RESIZE_READ_FLAG_TABLE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x88,
    };
    pub const SYS_SET_READ_FLAG_BIT: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x89,
    };
    pub const SYS_SET_READ_FLAG_RANGE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x8A,
    };
    pub const SYS_QUERY_READ_FLAG_BIT: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x8B,
    };

    pub const SYS_LOAD_GLOBAL_USER_DATA: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x80,
    };
    pub const SYS_SAVE_GLOBAL_USER_DATA: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x81,
    };
    pub const SYS_WRITE_GLOBAL_DATA_BLOCK: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x82,
    };
    pub const SYS_READ_GLOBAL_DATA_BLOCK: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x83,
    };
    pub const SYS_RESET_STRUCTURED_HISTORY: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x90,
    };
    pub const SYS_STRUCTURED_HISTORY_COUNT: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x91,
    };
    pub const SYS_APPEND_STRUCTURED_HISTORY_FIELDS: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x94,
    };
    pub const SYS_READ_STRUCTURED_HISTORY: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x95,
    };
    pub const SYS_APPEND_STRUCTURED_HISTORY_RECORD: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x96,
    };
    pub const SYS_READ_STRUCTURED_HISTORY_EXTENDED: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x97,
    };
    pub const SYS_INDEXED_RECORD_OPEN: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x98,
    };
    pub const SYS_INDEXED_RECORD_CLOSE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x99,
    };
    pub const SYS_INDEXED_RECORD_COUNT: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x9A,
    };
    pub const SYS_INDEXED_RECORD_PUSH: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x9C,
    };
    pub const SYS_INDEXED_RECORD_LOAD: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x9D,
    };
    pub const SYS_INDEXED_RECORD_REMOVE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0x9E,
    };
    pub const SYS_POLL_QUEUED_EVENT: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xA0,
    };
    pub const SYS_POST_QUEUED_EVENT: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xA1,
    };
    pub const SYS_SET_REGISTERED_OBJECT_STATE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xA8,
    };
    pub const SYS_GET_REGISTERED_OBJECT_STATE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xA9,
    };
    pub const SYS_QUEUE_REGISTERED_OBJECT_MESSAGE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xAC,
    };
    pub const SYS_SET_SYSTEM_MODE_FLAG: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xAF,
    };
    pub const SYS_CREATE_EXCLUSION_SECTION: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xB0,
    };
    pub const SYS_DELETE_EXCLUSION_SECTION: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xB1,
    };
    pub const SYS_WAIT_EXCLUSION_SECTION: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xB4,
    };
    pub const SYS_LEAVE_EXCLUSION_SECTION: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xB5,
    };
    pub const SYS_QUERY_EXCLUSION_SECTION: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xB6,
    };

    /// Target handler constructs CProcEncodeData from destination, source, and length; completion publishes the encoded length.
    pub const SYS_ENCODE_DATA: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xC0,
    };
    /// Target handler pops source first and destination second, then calls the synchronous SDC decoder and pushes its byte count.
    pub const SYS_DECODE_SDC_BUFFER: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xC1,
    };
    /// Target handler constructs CProcEncodeStruct, applying DCFS record transform and SDC compression asynchronously.
    pub const SYS_ENCODE_STRUCT_ARRAY: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xC4,
    };
    /// Target handler synchronously performs SDC decompression followed by DCFS reconstruction and returns the record count.
    pub const SYS_DECODE_STRUCT_ARRAY: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xC5,
    };
    /// Target constructs DCProcDecodeData. SDC streams are exact; the non-SDC target decoder remains represented by a bounded raw-copy compatibility path.
    pub const SYS_DECODE_DATA: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xCF,
    };
    /// Target allocates a fixed-record keyed table, writes an integer handle, and rejects record sizes <=1.
    pub const SYS_RECORD_TABLE_OPEN: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xD0,
    };
    /// Target removes the integer-handle table; a missing table returns 0x80000002.
    pub const SYS_RECORD_TABLE_CLOSE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xD1,
    };
    /// Target inserts or replaces one fixed-size record by string key.
    pub const SYS_RECORD_TABLE_INSERT: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xD2,
    };
    /// Target distinguishes missing table from missing key.
    pub const SYS_RECORD_TABLE_REMOVE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xD3,
    };
    /// Target fetches by key or insertion-order index and copies the fixed-size record.
    pub const SYS_RECORD_TABLE_FETCH: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xD4,
    };
    /// Target clears all string namespaces except protected table id 0x80000000.
    pub const SYS_CLEAR_STRING_NAMESPACES: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xD8,
    };
    /// Target returns the number of interned strings in one namespace.
    pub const SYS_STRING_NAMESPACE_COUNT: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xD9,
    };
    /// Target replaces a namespace from a packed string sequence or removes it when count is zero.
    pub const SYS_REPLACE_STRING_NAMESPACE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xDA,
    };
    /// Target serializes one namespace including each NUL terminator.
    pub const SYS_SERIALIZE_STRING_NAMESPACE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xDB,
    };
    /// Target interns/appends a string in one namespace with no graph-resource side effect.
    pub const SYS_INTERN_STRING_NAMESPACE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xDC,
    };
    /// Target reports the encoded byte length of one namespace entry without copying it.
    pub const SYS_STRING_NAMESPACE_ENTRY_LENGTH: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xDE,
    };
    /// Target launches a process, waits for it, and optionally restores the parent window.
    pub const SYS_LAUNCH_PROCESS_WAIT: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xE0,
    };
    /// Target stores restart fields, destroys the main window, returns native control status 6, and launches after WinMain cleanup.
    pub const SYS_RESTART_WITH_COMMAND: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xE1,
    };
    /// Target launches and waits, then additionally waits for the named uninstaller synchronization object.
    pub const SYS_LAUNCH_PROCESS_WAIT_UNINSTALLER: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xE2,
    };
    /// Target invokes ShellExecute with the open verb.
    pub const SYS_SHELL_OPEN: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xE3,
    };
    /// Target writes the literal Tayutama2TV string.
    pub const SYS_WRITE_GAME_ID: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xE8,
    };
    /// Target hashes bytes with h=byte+233*h and four rolling tail accumulators.
    pub const SYS_HASH_FILE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xE9,
    };
    /// Target stores the product used by the uninstaller synchronization message.
    pub const SYS_SET_UNINSTALLER_PRODUCT: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xEA,
    };
    /// Target opens a Win32 modal input dialog. Portable hosts explicitly return cancel when no equivalent UI is available.
    pub const SYS_SHOW_INPUT_DIALOG: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xF0,
    };
    /// Target opens a six-argument installer dialog; individual visual field names remain platform-bound.
    pub const SYS_SHOW_INSTALLER_DIALOG: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xF1,
    };
    /// Target executes the main installer workflow. Portable runtime preserves ABI and reports unsupported/cancel rather than false success.
    pub const SYS_RUN_INSTALLER_WORKFLOW: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xF2,
    };
    /// Target executes the shortcut installer workflow; portable runtime preserves ABI and returns unsupported.
    pub const SYS_RUN_SHORTCUT_INSTALLER_WORKFLOW: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xF3,
    };
    /// Target deletes files listed in root\uninst.lst except directives, the list itself, and explicit exclusions.
    pub const SYS_REMOVE_UNINSTALL_LISTED_FILES: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xF4,
    };
    /// Target appends missing entries to root\uninst.lst.
    pub const SYS_APPEND_UNINSTALL_LIST_ENTRIES: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xF5,
    };
    /// Target removes desktop/program-menu shortcuts and optionally the group.
    pub const SYS_REMOVE_INSTALLER_SHORTCUTS: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xF6,
    };
    /// Target creates a Windows .lnk shortcut. Portable hosts explicitly report unavailable.
    pub const SYS_CREATE_SHORTCUT: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xF7,
    };
    /// Target reads HKLM Software\vendor\product InstalledFolder. Portable non-Windows hosts report unavailable.
    pub const SYS_READ_INSTALLED_FOLDER: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xF8,
    };
    /// Target deletes HKLM Software\vendor\product.
    pub const SYS_DELETE_INSTALLED_REGISTRY_KEY: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xF9,
    };
    /// Target reads WindowsDir\file and accepts only NUL-terminated content ending in a backslash before the terminator.
    pub const SYS_READ_WINDOWS_PATH_FILE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xFA,
    };
    /// Target calls its special-folder helper with selector zero and writes the Windows directory.
    pub const SYS_WRITE_WINDOWS_DIRECTORY: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xFB,
    };
    /// Target registers a Windows file association. Portable non-Windows hosts report unavailable.
    pub const SYS_REGISTER_FILE_ASSOCIATION: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xFC,
    };
    /// Target returns the launcher mode global established by startup. Portable runtime exposes the configured compatibility value.
    pub const SYS_QUERY_LAUNCHER_MODE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xFD,
    };
    /// Target handler pushes constant 1.
    pub const SYS_INSTALLER_FEATURE_AVAILABLE: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xFE,
    };

    pub const SYS_COPY_INDEXED_NAMESPACE_RECORD: NativeOpcode = NativeOpcode {
        group: 0x80,
        id: 0xDD,
    };

    // System81 extended dispatcher: target-confirmed selector contracts.
    /// Target sub_498820 validates 50..60000 and stores the engine-clock discontinuity threshold.
    pub const SYS81_SET_CLOCK_JUMP_THRESHOLD: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x04,
    };
    /// Target sub_48E590 copies one of five two-DWORD coordinate transform slots.
    pub const SYS81_COPY_COORDINATE_SLOT: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x07,
    };
    /// Target calls GetUserNameA with a 257-byte capacity.
    pub const SYS81_GET_USER_NAME: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x08,
    };
    /// Target calls GetComputerNameA with a 16-byte capacity.
    pub const SYS81_GET_COMPUTER_NAME: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x09,
    };
    /// Target sub_46F720 gates extended CPUID leaves 0x80000002..4 and normalizes whitespace.
    pub const SYS81_GET_CPU_BRAND: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x0A,
    };
    /// Target sub_45E490 normalizes CPU text and copies four cached 16-bit signature fields as DWORDs.
    pub const SYS81_GET_CPU_DISPLAY_INFO: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x0B,
    };
    /// Target copies cached GetVersionExA data.
    pub const SYS81_GET_OS_VERSION: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x0C,
    };
    /// Target calls GlobalMemoryStatusEx and shifts both physical-memory values right by 20.
    pub const SYS81_GET_PHYSICAL_MEMORY_MB: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x0D,
    };
    /// Target sub_45E580 applies the wide/tall desktop halving rules.
    pub const SYS81_GET_ADJUSTED_DESKTOP_DIMENSIONS: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x0E,
    };
    /// Target directly returns IsIconic(hWndParent).
    pub const SYS81_IS_MAIN_WINDOW_MINIMIZED: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x0F,
    };
    /// Target sub_46DA40 returns the old dword_518CAC[6*index] then stores the replacement.
    pub const SYS81_SWAP_INPUT_BINDING: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x10,
    };
    /// Target directly calls GetKeyboardState.
    pub const SYS81_COPY_KEYBOARD_STATE: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x11,
    };
    /// Target sub_46D540 controls the raw keyboard polling override consumed by sub_46D560.
    pub const SYS81_SET_KEYBOARD_POLLING_OVERRIDE: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x14,
    };
    /// Target clears the linked history and accepts capacities up to 512.
    pub const SYS81_CONFIGURE_POINTER_HISTORY: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x16,
    };
    /// Target traverses the pointer-history linked list newest-first.
    pub const SYS81_COPY_POINTER_HISTORY: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x17,
    };
    /// Target calls RegisterTouchWindow or UnregisterTouchWindow for hWndParent.
    pub const SYS81_REGISTER_TOUCH_INPUT: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x18,
    };
    /// Target copies each touch record as six DWORDs.
    pub const SYS81_COPY_TOUCH_RECORDS: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x19,
    };
    /// Target sub_460E40 writes one of 36 controller wake values.
    pub const SYS81_SET_CONTROLLER_WAKE_ENTRY: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x1B,
    };
    /// Target sub_460E60 aggregates DirectInput device state and clamps three axes to -1024..1024.
    pub const SYS81_QUERY_CONTROLLER_STATE: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x1D,
    };
    /// Target posts the matching synthetic mouse down/up messages.
    pub const SYS81_INJECT_MOUSE_CLICK: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x1E,
    };
    /// Target sub_4011B0 returns 1 invalid mode, 2 duplicate path, 3 open failure, or 0.
    pub const SYS81_OPEN_RESOURCE_STREAM: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x28,
    };
    /// Target queues a close operation; missing handles map to status 4.
    pub const SYS81_QUEUE_RESOURCE_CLOSE: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x29,
    };
    /// Target sub_4014A0 queues a read request on the stream worker list.
    pub const SYS81_QUEUE_RESOURCE_READ: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x2A,
    };
    /// Target sub_4014C0 queues a no-buffer stream operation at the supplied offset.
    pub const SYS81_QUEUE_RESOURCE_SEEK: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x2B,
    };
    /// Target opens the file abstraction, queries three FILETIMEs and converts each to SYSTEMTIME.
    pub const SYS81_GET_FILE_TIMES: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x2C,
    };
    /// Target converts three SYSTEMTIMEs to FILETIME and applies them to the opened file.
    pub const SYS81_SET_FILE_TIMES: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x2D,
    };
    /// Target creates, reopens and deletes a temporary file in the directory.
    pub const SYS81_TEST_PATH_WRITABLE: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x2F,
    };
    /// Target chooses synchronous sub_467F50 or DCProcReadBinary from the one-shot async flag.
    pub const SYS81_READ_RESOURCE_BINARY: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x30,
    };
    /// Target WinINet helper distinguishes initialization, open, read and short-read failures.
    pub const SYS81_INTERNET_READ: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x31,
    };
    /// Target uses unbuffered/direct file I/O and validates a drive-letter path.
    pub const SYS81_DIRECT_FILE_READ: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x32,
    };
    /// Target sub_468310 returns the resource size.
    pub const SYS81_RESOURCE_SIZE: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x35,
    };
    /// Target maps GetDriveTypeA to BGI codes: fixed 1, removable 2, network 3, CD-ROM 4, RAM disk 5.
    pub const SYS81_ENUMERATE_DRIVE_TYPES: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x36,
    };
    /// Target queries free bytes and shifts right by 20.
    pub const SYS81_GET_DISK_FREE_MB: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x37,
    };
    /// Target converts parallel VM pointer tables for filter labels and extensions, then calls the Win32 dialog helper. The portable host preserves the same array/status contract through native platform choosers.
    pub const SYS81_SHOW_RESOURCE_FILE_DIALOG: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x38,
    };
    /// Target sub_467A60 enumerates matching archive/resource names into packed NUL strings.
    pub const SYS81_ENUMERATE_RESOURCES: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x39,
    };
    /// Target opens SHBrowseForFolder and converts the selected PIDL to a path. The portable host uses the platform folder chooser and writes the selected path through the same BP buffer contract.
    pub const SYS81_BROWSE_FOLDER: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x3A,
    };
    /// Target enumerates resources, joins names with newlines and opens a modal list. The portable host enumerates matching user resources and presents a blocking platform list acknowledgement.
    pub const SYS81_SHOW_RESOURCE_LIST: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x3B,
    };
    /// Target sub_4685D0 tests the resource abstraction rather than only the host filesystem.
    pub const SYS81_RESOURCE_EXISTS: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x3C,
    };
    /// Target sub_468670 calls GetVolumeInformationA.
    pub const SYS81_GET_VOLUME_LABEL: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x3D,
    };
    /// Target returns whether the query mechanism opened/succeeded and writes the device power state.
    pub const SYS81_QUERY_DEVICE_POWER_STATE: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x3E,
    };
    /// Target constructs DCTChildThread, validates sizes/index and returns CThread+8.
    pub const SYS81_CREATE_CHILD_THREAD: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x44,
    };
    /// Target sub_461040 validates slot and nonzero dimensions.
    pub const SYS81_CONFIGURE_SCREEN_MODE: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x60,
    };
    /// Target returns the configured mode, but mode 2 falls back to zero when adjusted desktop dimensions are too small.
    pub const SYS81_QUERY_EFFECTIVE_DISPLAY_MODE: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x61,
    };
    /// Target sub_45E830 stores the monitor/adapter selection mode.
    pub const SYS81_SET_MONITOR_ADAPTER_MODE: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x62,
    };
    /// Target sub_45E860 stores the display/input configuration mode.
    pub const SYS81_SET_CONFIG_INPUT_MODE: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x63,
    };
    /// Target sub_45E770 updates logical dimensions and triggers render reconfiguration.
    pub const SYS81_CONFIGURE_LOGICAL_SCREEN_SIZE: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x64,
    };
    /// Target value is consumed by WM_WINDOWPOSCHANGING position override logic.
    pub const SYS81_SET_WINDOW_POSITION_OVERRIDE: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x65,
    };
    /// Target sub_498800 swaps dword_566A4C and returns the old value.
    pub const SYS81_SWAP_PAUSE_ON_DEACTIVATE: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x68,
    };
    /// Target registers/unregisters sixteen PrintScreen hotkeys for modifiers 0..15.
    pub const SYS81_SET_PRINT_SCREEN_HOTKEY_CAPTURE: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x69,
    };
    /// Target sub_464440 selects whether engine error text is captured.
    pub const SYS81_SET_ERROR_CAPTURE_MODE: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x6A,
    };
    /// Target sub_464480 copies dword_566004 when non-null and returns strlen+1.
    pub const SYS81_COPY_CAPTURED_ERROR: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x6B,
    };
    /// Target sub_460590 reads the D3DCAPS9 field at offset 0xCC.
    pub const SYS81_QUERY_PIXEL_SHADER_VERSION: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x6D,
    };
    /// Target directly returns the Microsoft CRT __uncaught_exception result.
    pub const SYS81_UNCAUGHT_EXCEPTION_ACTIVE: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x6E,
    };
    /// Target sub_460550 manages the D3DX bicubic scaler shader.
    pub const SYS81_SET_BICUBIC_SHADER: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0x6F,
    };
    /// Target sub_495C50 computes len(left)+len(right)-LCS(left,right).
    pub const SYS81_WIDE_STRING_DISTANCE: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0xB0,
    };
    /// Target converts single-byte and Shift-JIS double-byte code units without a Windows normalization pass.
    pub const SYS81_SHIFT_JIS_TO_UTF16: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0xB7,
    };
    /// Target creates a growable array of {pointer,size} blob slots.
    pub const SYS81_BLOB_TABLE_OPEN: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0xD0,
    };
    /// Target removes the matching table node and frees all blobs.
    pub const SYS81_BLOB_TABLE_CLOSE: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0xD1,
    };
    /// Target stores into a requested slot or the first free slot; allocation failure maps to 0x80000003.
    pub const SYS81_BLOB_TABLE_STORE: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0xD2,
    };
    /// Target clears and frees one occupied slot; invalid slot maps to 0x80000004.
    pub const SYS81_BLOB_TABLE_REMOVE: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0xD3,
    };
    /// Target copies one slot and reports 0x80000004 when absent.
    pub const SYS81_BLOB_TABLE_FETCH: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0xD4,
    };
    /// Target emits index/size pairs for occupied slots.
    pub const SYS81_BLOB_TABLE_ENUMERATE: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0xD5,
    };
    /// Target sub_4724B0 launches, waits while pumping BGI messages, and optionally reads exit code.
    pub const SYS81_LAUNCH_PROCESS_WAIT: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0xE0,
    };
    /// Target sub_401900 updates a 233-multiplier DWORD plus four byte accumulators.
    pub const SYS81_UPDATE_ROLLING_HASH: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0xE9,
    };
    /// Target sub_4A1440 pads with the MD5 constants/rounds and writes four digest DWORDs.
    pub const SYS81_MD5: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0xEA,
    };
    /// Target rejects pre-existing mutex names and returns a monotonic BGI handle.
    pub const SYS81_CREATE_NAMED_MUTEX: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0xEC,
    };
    /// Target releases/closes the native mutex and removes its BGI record.
    pub const SYS81_RELEASE_NAMED_MUTEX: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0xED,
    };
    /// Target constructs DCProcInstallation; initialization status zero installs the procedure, while 0x80000000..2 map to 1..3.
    pub const SYS81_RUN_INSTALLATION_PROCEDURE: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0xF2,
    };
    /// Target sub_4713F0 validates or creates a Win32 special-folder/user path.
    pub const SYS81_VALIDATE_OR_CREATE_USER_PATH: NativeOpcode = NativeOpcode {
        group: 0x81,
        id: 0xF7,
    };

    /// Target helpers sub_461D70/sub_461D80 coalesce pending redraw requests and preserve the stronger full-redraw request.
    pub const GRAPH90_REQUEST_REDRAW: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x00,
    };
    /// Target stores the supplied graph scheduler gate without converting it to an unrelated rendering property.
    pub const GRAPH90_SET_SCHEDULER_GATE: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x01,
    };
    /// Target validates 1..=1000, derives the millisecond interval, and resets its next frame deadline.
    pub const GRAPH90_SET_FRAME_RATE: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x02,
    };
    /// Target validates the byte limit and initializes/resets the native bitmap memory manager. Portable runtime enforces the limit but does not reproduce the allocator layout.
    pub const GRAPH90_INITIALIZE_BITMAP_MEMORY_MANAGER: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x03,
    };
    /// Target allocates a work bitmap using current display dimensions and native format.
    pub const GRAPH90_CREATE_WORK_BITMAP: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x04,
    };
    /// Target creates a work bitmap and registers its 16-bit render priority.
    pub const GRAPH90_CREATE_PRIORITIZED_WORK_BITMAP: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x05,
    };
    /// Target stores the graph center coordinates.
    pub const GRAPH90_SET_CENTER: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x06,
    };
    /// Target stores the default duration used by later graph control procedures.
    pub const GRAPH90_SET_SYNC_LOAD_HOLD: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x07,
    };
    /// Target stores the enable gate and invalidates/rechecks the display chain.
    pub const GRAPH90_SET_DISPLAY_ENABLED: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x08,
    };
    /// Target validates and stores the default graph priority.
    pub const GRAPH90_SET_DEFAULT_PRIORITY: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x09,
    };
    /// Target writes two globals controlling redraw requests after display-object update completion; it is not a global coordinate offset.
    pub const GRAPH90_SET_OBJECT_UPDATE_REDRAW_POLICY: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x0A,
    };
    /// Target writes the supplied bitmap handle to the bitmap manager current slot; it is not the current display object.
    pub const GRAPH90_SET_CURRENT_BITMAP: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x0B,
    };
    /// Target stores two display settings and invalidates sixteen display-object records.
    pub const GRAPH90_SET_DISPLAY_OPTIONS: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x0C,
    };
    /// Target derives layouts (2,1,2), (4,2,4), (6,3,8), or (8,4,16) and rebuilds registered fonts.
    pub const GRAPH90_SET_RASTER_FORMAT_MODE: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x0D,
    };
    /// Target deduplicates by face/height/weight/italic and updates the registered font id.
    pub const GRAPH90_REGISTER_FONT: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x0E,
    };
    /// Target stores the packed RGB color used by bitmap alpha-unblend paths.
    pub const GRAPH90_SET_BITMAP_UNBLEND_COLOR: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x0F,
    };
    /// Target uses a synchronous cache hit path or installs CProcLoadBitmap and returns native scheduler status 2.
    pub const GRAPH90_LOAD_BITMAP: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x10,
    };
    /// Target creates a bitmap with explicit dimensions and format.
    pub const GRAPH90_CREATE_BITMAP: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x11,
    };
    /// Target releases one bitmap registry slot and pushes success.
    pub const GRAPH90_RELEASE_BITMAP: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x12,
    };
    /// Target clears/fills the selected bitmap with the supplied packed color.
    pub const GRAPH90_FILL_BITMAP: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x13,
    };
    /// Target creates a bitmap from caller memory. The VM bridge preserves BP memory ownership and byte count.
    pub const GRAPH90_CREATE_BITMAP_FROM_PIXELS: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x14,
    };
    /// Target copies bitmap bytes to caller memory subject to capacity and writes the copied count.
    pub const GRAPH90_COPY_BITMAP_PIXELS: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x15,
    };
    /// Returns the target registry metadata written as stride/width/height/format/pixel-size fields.
    pub const GRAPH90_QUERY_BITMAP_INFO: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x16,
    };
    /// Target returns 0 for compatible, 1 for missing, and 2 for incompatible, with its format-2 to format-1 conversion allowance.
    pub const GRAPH90_VALIDATE_BITMAP_FORMAT: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x17,
    };
    /// Validates existing compatible bitmaps before clipped mode/parameter dispatch.
    pub const GRAPH90_BLIT_BITMAP: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x18,
    };
    /// Target performs a three-resource bitmap synthesis.
    pub const GRAPH90_SYNTHESIZE_BITMAP: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x19,
    };
    /// Target performs a six-argument multi-source composite.
    pub const GRAPH90_COMPOSITE_BITMAPS: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x1A,
    };
    /// Target performs a four-argument bitmap copy mode.
    pub const GRAPH90_COPY_BITMAP: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x1B,
    };
    /// Target performs a ten-argument scaled-region operation.
    pub const GRAPH90_SCALE_BITMAP_REGION: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x1C,
    };
    /// Target performs a four-argument bitmap transform/copy.
    pub const GRAPH90_TRANSFORM_BITMAP: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x1D,
    };
    /// Target performs an eight-argument rectangular bitmap operation.
    pub const GRAPH90_BLIT_BITMAP_REGION: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x1E,
    };
    /// Allocates detached source-format storage and copies a requested source rectangle.
    pub const GRAPH90_CREATE_BITMAP_REGION: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x1F,
    };
    /// Target validates control selectors and installs CProcCtrlDspObj through sub_491B40.
    pub const GRAPH90_START_OBJECT_CONTROL: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x20,
    };
    /// Nine-argument XY/curve CProcCtrlDspObj form recovered at sub_47A890/sub_431D50.
    pub const GRAPH90_START_NODE_CONTROL: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x21,
    };
    /// Seven-argument alpha-only CProcCtrlDspObj form recovered at sub_47A790/sub_431D10.
    pub const GRAPH90_START_OBJECT_CONTROL_EX: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x22,
    };
    /// Extended alternate CProcCtrlDspObj constructor path.
    pub const GRAPH90_START_NODE_CONTROL_EX: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x23,
    };
    /// Target installs the special display-object control procedure through sub_491C40.
    pub const GRAPH90_START_SPECIAL_OBJECT_CONTROL: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x24,
    };
    /// Target installs the motion-control CProcCtrlDspObj path through sub_491D60.
    pub const GRAPH90_START_OBJECT_MOTION_CONTROL: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x28,
    };
    /// Target installs CProcCtrlDspObjBC through sub_491E60; sub_432490 builds three natural CSpline axes from the current vector plus script points.
    pub const GRAPH90_START_SPLINE_OBJECT_CONTROL: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x29,
    };
    /// Target installs CProcShakeDspObj through sub_491F90/sub_43C7A0; this is the display-object shake procedure, not a spline control.
    pub const GRAPH90_START_SHAKE_OBJECT_CONTROL: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x2C,
    };
    /// Target vtable+4 writes CDspObj+0x14 and propagates the independent draw gate through children.
    pub const GRAPH90_SET_OBJECT_DRAW_ENABLED: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x30,
    };
    /// Target writes CDspObj+0x04 and updates the inherited enabled gate.
    pub const GRAPH90_SET_OBJECT_ENABLED: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x31,
    };
    /// Target dispatches object positioning through class-specific vtable slot +44.
    pub const GRAPH90_SET_OBJECT_POSITION: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x33,
    };
    /// Target sub_41B660 writes CDspObj+0xB0 and propagates the mask-alpha value.
    pub const GRAPH90_SET_OBJECT_MASK_ALPHA: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x34,
    };
    /// Target invokes vtable+80 in mode zero and stores value<<16 at CDspObj+0xB8; it is not scale_x.
    pub const GRAPH90_SET_OBJECT_FIXED_PARAMETER: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x35,
    };
    /// Target sub_41B320 writes CDspObj+0x40/+0x44 and propagates the secondary offset.
    pub const GRAPH90_SET_OBJECT_SECONDARY_OFFSET: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x36,
    };
    /// Target vtable+56 writes CDspObj+0x38/+0x3C and propagates the primary offset.
    pub const GRAPH90_SET_OBJECT_PRIMARY_OFFSET: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x37,
    };
    /// Target forwards the four source-order arguments to sub_4438B0 and maps its object/property errors.
    pub const GRAPH90_SET_OBJECT_PROPERTY: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x38,
    };
    /// Target sub_41B6A0 writes CDspObj+0xB4 and propagates the alpha multiplier.
    pub const GRAPH90_SET_OBJECT_ALPHA_MULTIPLIER: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x39,
    };
    /// Target vtable+84 writes CDspObj+0x1C; the object manager reinserts the object when its packed depth changes.
    pub const GRAPH90_SET_OBJECT_PRIORITY: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x3A,
    };
    /// Target builds or clears the CDspObj hit mask and reports distinct missing-object/missing-bitmap errors.
    pub const GRAPH90_SET_OBJECT_HIT_MASK_BITMAP: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x3C,
    };
    /// Target obtains object and pointer positions, converts to local coordinates, and invokes CDspObj vtable+100 with hit-mask testing enabled.
    pub const GRAPH90_HIT_TEST_OBJECT_AT_POINTER: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x3D,
    };
    /// All CDspObj-derived vtables in this target route slot +108 to sub_41BE90 (0x80000001), which the public handler maps to its target error path.
    pub const GRAPH90_INVOKE_UNSUPPORTED_OBJECT_EXTENSION: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x3F,
    };

    /// Target graph selector whose backing virtual setter stores the native
    /// transparency parameter in `CDspObj+0xAC`.
    pub const GRAPH_SET_OBJECT_ALPHA: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x32,
    };
    // Target-confirmed System90 display-object families (0x40-0x7A).
    /// Target sub_47B5D0 validates the bitmap and sub_462130 stores it in the current display object, not in the global primary-bitmap slot.
    pub const GRAPH90_SET_CURRENT_OBJECT_BITMAP: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x40,
    };
    /// Target sub_47B630 forwards two bitmap handles and the transparency parameter to sub_462140 for the current display object.
    pub const GRAPH90_CONFIGURE_CURRENT_OBJECT_DUAL_BITMAP: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x41,
    };
    /// Target sub_47B6C0 configures four bitmap resources plus a coordinate pair on the current display object.
    pub const GRAPH90_CONFIGURE_CURRENT_OBJECT_QUAD_BITMAP: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x42,
    };
    /// Target sub_47B7C0/sub_462170 configures the current object resource set, validates the optional format-3 mask, and stores transparency.
    pub const GRAPH90_CONFIGURE_CURRENT_OBJECT_SPRITE_MASK: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x43,
    };
    /// Target sub_47B930 copies 2..32 resource handles from VM memory into the current object frame table and applies transparency.
    pub const GRAPH90_CONFIGURE_CURRENT_OBJECT_FRAME_TABLE: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x44,
    };
    /// Target sub_47BA80/sub_4621B0 configures a resource triplet, transparency, and one extension field on the current display object.
    pub const GRAPH90_CONFIGURE_CURRENT_OBJECT_RESOURCE_TRIPLET: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x45,
    };
    /// Target sub_47BBE0/sub_4621D0 validates a bitmap, a two-value mode, and transparency.
    pub const GRAPH90_CONFIGURE_CURRENT_OBJECT_BITMAP_MODE: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x46,
    };
    /// Target sub_47BCC0/sub_4621E0 binds a bitmap and auxiliary resource to a nonempty VM record table and stores transparency.
    pub const GRAPH90_CONFIGURE_CURRENT_OBJECT_VM_EFFECT: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x47,
    };
    /// Target sub_47BE20/sub_462200 validates the bitmap and minimum dimensions and stores size plus position.
    pub const GRAPH90_CONFIGURE_CURRENT_OBJECT_BITMAP_SIZE_POSITION: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x48,
    };
    /// Target sub_47BEF0/sub_462220 binds a bitmap and a nonzero width/height pair to the current object.
    pub const GRAPH90_CONFIGURE_CURRENT_OBJECT_BITMAP_SIZE: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x49,
    };
    /// Target sub_47BF90/sub_462230 configures two resources; the mode must be zero and the final flag is boolean-like.
    pub const GRAPH90_CONFIGURE_CURRENT_OBJECT_BLIT_SOURCES: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x4A,
    };
    /// Target sub_47C090/sub_462330 updates two current-object render controls and invalidates the object.
    pub const GRAPH90_SET_CURRENT_OBJECT_RENDER_CONTROLS: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x4C,
    };
    /// Target sub_47C0C0/sub_462340 returns the current display object mode field at +0x134; it is not a render-target handle.
    pub const GRAPH90_GET_CURRENT_OBJECT_MODE: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x4D,
    };
    /// Target sub_462350 allocates one of 512 CDspObjSprite slots and encodes the reusable slot as 0x80000000 | index. The manager create path sub_43E510 separately increments a monotonic Sprite construction counter and passes its previous value in ECX to sub_4256C0 -> CDspObj::CDspObj, producing CDspObj+0x18 sort_class=2 and +0x20 sort_index=construction counter; +0x20 is therefore not the public slot and is not reused after release/recreate.
    pub const GRAPH90_CREATE_SPRITE_OBJECT: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x50,
    };
    /// Target rejects sprites still owned by active control procedures before releasing the CDspObjSprite slot.
    pub const GRAPH90_RELEASE_SPRITE_OBJECT: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x51,
    };
    /// Target sub_47C170/sub_462550 rebuilds/refreshes an existing sprite object. The public ABI consumes four additional values, but the target core ignores them.
    pub const GRAPH90_REFRESH_SPRITE_OBJECT: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x53,
    };
    /// Target sub_47C1F0/sub_462540 writes the sprite enabled gate.
    pub const GRAPH90_SET_SPRITE_ENABLED: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x54,
    };
    /// Target sub_47C230/sub_462520 installs or clears the sprite auxiliary bitmap and validates descriptor format.
    pub const GRAPH90_SET_SPRITE_AUX_BITMAP: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x55,
    };
    /// Target sub_47C2D0/sub_462370 configures sprite mode 0 with one bitmap and common display state.
    pub const GRAPH90_CONFIGURE_SPRITE_SINGLE_BITMAP: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x56,
    };
    /// Target sub_47C3E0/sub_462510 replaces the current CDspObjSprite primary bitmap and rebuilds mode-specific geometry for modes 0/2/5/6.
    pub const GRAPH90_REPLACE_SPRITE_BITMAP: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x57,
    };
    /// Compatibility alias for callers built against the earlier provisional
    /// name. Opcode 0x90:0x57 is a primary-bitmap replacement, not an
    /// offscreen-render operation.
    pub const GRAPH90_RENDER_SPRITE_TO_BITMAP: NativeOpcode = GRAPH90_REPLACE_SPRITE_BITMAP;
    /// Target sub_47C470/sub_462390 configures an existing CDspObjSprite as mode 1. It does not allocate a generic transition node.
    pub const GRAPH90_CONFIGURE_SPRITE_DUAL_BITMAP: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x58,
    };
    /// Target sub_47C5D0/sub_4623C0 configures sprite mode 2 as a full-bitmap affine transform with an ordinary CDspObj anchor and common display state.
    pub const GRAPH90_CONFIGURE_SPRITE_SCALED_BITMAP: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x59,
    };
    /// Target sub_47C770/sub_462400 configures sprite mode 3 with a masked bitmap and fixed-point parameter.
    pub const GRAPH90_CONFIGURE_SPRITE_MASKED_BITMAP: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x5A,
    };
    /// Target sub_47C8F0/sub_462430 configures sprite mode 4 with a format-6 effect resource and nonempty VM record table.
    pub const GRAPH90_CONFIGURE_SPRITE_VM_EFFECT: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x5B,
    };
    /// Target sub_47CC10/sub_462460 configures sprite mode 5 with three position/transform values, primary/optional secondary bitmaps, transition/fixed/transform fields, then blend, transparency, and priority.
    pub const GRAPH90_CONFIGURE_SPRITE_TRANSFORM_MODE5: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x5C,
    };
    /// Target sub_47CF60/sub_4624B0 configures sprite mode 6 with three position/transform values, primary/optional secondary bitmaps, transition/fixed/transform fields, then blend, transparency, and priority.
    pub const GRAPH90_CONFIGURE_SPRITE_TRANSFORM_MODE6: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x5D,
    };
    /// Target sub_462560 allocates one of eight CDspObjFilter slots and encodes the reusable slot as 0x90000000 | index. The concrete constructor uses CDspObj sort_class=7 and a separate per-registry monotonic construction counter for CDspObj+0x20, so release/recreate does not reuse the native sort index.
    pub const GRAPH90_CREATE_FILTER_OBJECT: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x60,
    };
    /// Target sub_47D320/sub_462570 releases a CDspObjFilter. The prior GraphObjectUpdate interpretation was incorrect.
    pub const GRAPH90_RELEASE_FILTER_OBJECT: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x61,
    };
    /// Target sub_47D350/sub_4625A0 writes the filter enabled gate.
    pub const GRAPH90_SET_FILTER_ENABLED: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x64,
    };
    /// Target sub_47D390 calls the full filter configurator with no mask, preserving filter parameter, transparency, and priority.
    pub const GRAPH90_CONFIGURE_FILTER: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x65,
    };
    /// Target sub_47D400/sub_462580 consumes one reserved ABI slot, then configures a filter with an optional format-3 mask whose dimensions must match.
    pub const GRAPH90_CONFIGURE_FILTER_WITH_MASK: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x66,
    };
    /// Target sub_462680 allocates one of eight CDspObjMap slots and encodes the reusable slot as 0xA0000000 | index. sub_423920 constructs the object with CDspObj sort_class=1 and the manager supplies a separate per-registry monotonic construction counter for CDspObj+0x20; that native sort index is not the reusable handle slot.
    pub const GRAPH90_CREATE_MAP_OBJECT: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x70,
    };
    /// Target sub_47D6A0/sub_462690 releases a CDspObjMap slot.
    pub const GRAPH90_RELEASE_MAP_OBJECT: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x71,
    };
    /// Target sub_47D6D0/sub_4626A0 writes the map enabled gate.
    pub const GRAPH90_SET_MAP_ENABLED: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x74,
    };
    /// Target sub_47D710/sub_4626B0 configures map resource, position, blend, transparency, and priority.
    pub const GRAPH90_CONFIGURE_MAP_OBJECT: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x75,
    };
    /// Target sub_47D820/sub_4626D0 initializes a map grid; each count is at most 256 and total pixel size is bounded by 1024x768.
    pub const GRAPH90_INITIALIZE_MAP_GRID: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x76,
    };
    /// Target sub_47D900/sub_4626F0 copies a u16 map-data rectangle from VM memory into the map object.
    pub const GRAPH90_UPLOAD_MAP_TILE_DATA: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x78,
    };
    /// Target sub_47D9A0/sub_462710 updates the map source viewport, cell offsets, and wrapping selector.
    pub const GRAPH90_SET_MAP_VIEWPORT: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x79,
    };
    /// Target sub_47DA70/sub_462730 scans the first grid; when an entry equals tile_id, it writes the bitwise complement of the current second-grid entry. Target assembly advances the second-grid cursor once per cell plus one extra step after a match.
    pub const GRAPH90_REPLACE_MAP_TILE_ID: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x7A,
    };

    /// Target sub_4405B0 allocates one of 16 CDspObjWindow slots and returns 0xB0000000 | slot.
    pub const GRAPH90_CREATE_WINDOW_OBJECT: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x80,
    };
    /// Target rejects owned/procedure-bound windows before releasing the tagged slot.
    pub const GRAPH90_RELEASE_WINDOW_OBJECT: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x81,
    };
    /// Target stores one of six permutations in CDspObjWindow+0x3C0/+0x3C4/+0x3C8.
    pub const GRAPH90_SET_WINDOW_COMPOSITION_ORDER: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x82,
    };
    /// Target invokes the window draw-enable virtual setter and invalidates on change.
    pub const GRAPH90_SET_WINDOW_DRAW_ENABLED: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x84,
    };
    /// Target configures window position, blend, transparency and priority; one validated ABI slot is ignored.
    pub const GRAPH90_CONFIGURE_WINDOW_OBJECT: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x85,
    };
    /// Target stores CDspObjWindow+956 and switches isolated precomposition of backing/frame/text passes.
    pub const GRAPH90_SET_WINDOW_ISOLATED_COMPOSITION: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x87,
    };
    /// Target validates and stores the inclusive window valid rectangle at CDspObjWindow+416..+428.
    pub const GRAPH90_SET_WINDOW_VALID_REGION: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x88,
    };
    /// Target copies CDspObjWindow+416..+428 to caller BP memory.
    pub const GRAPH90_GET_WINDOW_VALID_REGION: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x89,
    };
    /// Target installs CProcDspMsg and returns scheduler status 2.
    pub const GRAPH90_START_WINDOW_MESSAGE_PROCEDURE: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x90,
    };
    /// Target rebuilds the global caret-frame array from a caller bitmap table.
    pub const GRAPH90_CONFIGURE_MESSAGE_CARET_FRAMES: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x98,
    };
    /// Target writes the message caret frame interval global.
    pub const GRAPH90_SET_MESSAGE_CARET_FRAME_DELAY: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x99,
    };
    /// Target stores caret coordinate mode and X/Y values; mode 1 is absolute and other modes are window-relative.
    pub const GRAPH90_SET_MESSAGE_CARET_POSITION: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x9A,
    };
    /// Target writes the message glyph shadow/effect enable global.
    pub const GRAPH90_SET_TEXT_SHADOW_ENABLED: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x9C,
    };
    /// Target validates two percentage offsets and a 0..=256 shadow concentration parameter.
    pub const GRAPH90_SET_TEXT_SHADOW_PARAMETERS: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x9D,
    };
    /// Target slices a horizontal bitmap strip into at most 255 private full-width glyph slots beginning at 0xFF01.
    pub const GRAPH90_REGISTER_FULLWIDTH_GLYPH_STRIP: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x9E,
    };

    /// Target installs CProcSelectItem after validating a 1..=16 item grid.
    pub const GRAPH90_START_ITEM_SELECTION: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xA0,
    };
    /// Target renders the base item-selection text grid immediately without installing a procedure.
    pub const GRAPH90_DRAW_ITEM_SELECTION_GRID: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xA1,
    };
    /// Target installs CProcSelectItemEx with two optional overlay bitmap slots.
    pub const GRAPH90_START_ITEM_SELECTION_EX: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xA2,
    };
    /// Target installs CProcSelectItemExBlink with its initial 20-tick interaction gate.
    pub const GRAPH90_START_ITEM_SELECTION_EX_BLINK: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xA3,
    };
    /// Stores the two alternating selected-item text styles.
    pub const GRAPH90_SET_ITEM_SELECTION_HIGHLIGHT_STYLES: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xA4,
    };
    /// Stores the CProcSelectItem input mask.
    pub const GRAPH90_SET_ITEM_SELECTION_INPUT_MASK: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xA5,
    };
    /// Stores the four process-global item-selection navigation parameters.
    pub const GRAPH90_SET_ITEM_SELECTION_NAVIGATION_PARAMETERS: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xA6,
    };
    /// Copies a fixed 16-DWORD column-layout table into one CDspObjWindow.
    pub const GRAPH90_SET_ITEM_SELECTION_COLUMN_LAYOUT: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xA7,
    };
    /// Stores the duplicated input-poll gate consumed by selection/icon procedures.
    pub const GRAPH90_SET_INTERACTIVE_PROCEDURE_POLL_GATE: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xAF,
    };
    /// Target installs CProcSelectIcon from 16-byte icon records.
    pub const GRAPH90_START_ICON_SELECTION: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xB0,
    };
    /// Target installs CProcSelectIconEx from 64-byte extended icon records.
    pub const GRAPH90_START_ICON_SELECTION_EX: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xB1,
    };
    /// Draws a batch of base 16-byte icon records into a window.
    pub const GRAPH90_DRAW_ICON_BATCH: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xB4,
    };
    /// Projects 64-byte extended records to base icon records and draws them.
    pub const GRAPH90_DRAW_EXTENDED_ICON_BATCH: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xB5,
    };
    /// Parses and immediately applies a compact DCIPIcon layout to a window.
    pub const GRAPH90_APPLY_ICON_INPUT_LAYOUT: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xB6,
    };
    /// Parses and immediately applies an extended DCIPIconEx layout to a window.
    pub const GRAPH90_APPLY_ICON_INPUT_LAYOUT_EX: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xB7,
    };
    /// Creates a registered DCIPIcon processor and returns its opaque handle.
    pub const GRAPH90_CREATE_ICON_INPUT_PROCESSOR: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xB8,
    };
    /// Releases a registered DCIPIcon processor handle.
    pub const GRAPH90_RELEASE_ICON_INPUT_PROCESSOR: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xB9,
    };
    /// Configures base DCIPIcon from a compact descriptor; normal, selected and pointer-hover resource slots are distinct.
    pub const GRAPH90_CONFIGURE_ICON_INPUT_PROCESSOR: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xBA,
    };
    /// Writes [running, group, item, action_state, local_x, local_y].
    pub const GRAPH90_GET_ICON_INPUT_STATE: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xBC,
    };
    /// Writes the current DCIPIcon group index.
    pub const GRAPH90_GET_ICON_INPUT_CURRENT_GROUP: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xBD,
    };
    /// Writes one selected item index per DCIPIcon group.
    pub const GRAPH90_GET_ICON_INPUT_SELECTIONS: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xBE,
    };
    /// Destructively pops one queued [event,payload,parameter] record; hover changes are 0x10000002.
    pub const GRAPH90_POP_ICON_INPUT_EVENT: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xBF,
    };

    /// Target validates and decodes one BG resource into the selected bitmap slot.
    pub const GRAPH90_LOAD_BG_BITMAP_RESOURCE: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xC0,
    };
    /// Target mirrors a source bitmap horizontally (0) or vertically (1).
    pub const GRAPH90_FLIP_BITMAP: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xC2,
    };
    /// Target creates a ceil(width/2) by ceil(height/2) downsampled bitmap.
    pub const GRAPH90_DOWNSAMPLE_BITMAP_HALF: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xC3,
    };
    /// Target imports an external image through GDI+ into caller-owned BP memory.
    pub const GRAPH90_IMPORT_EXTERNAL_IMAGE: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xC4,
    };
    /// Target saves the current bitmap as BMP/JPEG/GIF/TIFF/PNG through GDI+.
    pub const GRAPH90_SAVE_BITMAP_TO_IMAGE_FILE: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xC5,
    };
    /// Target validates BG bytes and registers a lower-cased two-key cache entry.
    pub const GRAPH90_REGISTER_BG_RESOURCE_DATA: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xC6,
    };
    /// Target decodes a cached/archive BG resource and optionally consumes the cached bytes.
    pub const GRAPH90_LOAD_CACHED_BG_BITMAP: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xC7,
    };
    /// Target performs its general 18-argument geometric bitmap sampling operation.
    pub const GRAPH90_TRANSFORM_BITMAP_GENERAL: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xC8,
    };
    /// Target scales a source bitmap into a destination while preserving aspect ratio and centering it.
    pub const GRAPH90_SCALE_BITMAP_ASPECT_FIT: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xCA,
    };
    /// Target registers or removes a three-control-point, three-channel tone curve.
    pub const GRAPH90_REGISTER_TONE_CURVE: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xCC,
    };
    /// Target applies a registered tone curve plus color-film/monochrome effect parameters.
    pub const GRAPH90_APPLY_TONE_CURVE_EFFECT: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xCD,
    };
    /// Target encodes a bitmap into caller memory and writes the encoded byte count.
    pub const GRAPH90_ENCODE_BITMAP_TO_BUFFER: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xCE,
    };

    /// Target allocates one of 32 CDspObjKnob slots as 0xF0000000 | slot.
    pub const GRAPH90_CREATE_KNOB_OBJECT: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xD0,
    };
    pub const GRAPH90_RELEASE_KNOB_OBJECT: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xD1,
    };
    pub const GRAPH90_SET_KNOB_ENABLED: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xD4,
    };
    pub const GRAPH90_SET_KNOB_BASE_POSITION: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xD5,
    };
    pub const GRAPH90_SET_KNOB_POSITION: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xD6,
    };
    pub const GRAPH90_GET_KNOB_POSITION: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xD7,
    };
    pub const GRAPH90_SET_KNOB_MOVEMENT_PRECISION: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xD8,
    };
    pub const GRAPH90_SET_KNOB_MOVEMENT_RANGE: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xD9,
    };
    pub const GRAPH90_TAKE_KNOB_VERTICAL_EVENT: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xDA,
    };
    pub const GRAPH90_TAKE_CHANGED_KNOB_HANDLE: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xDB,
    };
    pub const GRAPH90_SET_KNOB_RELATIVE_MODE: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xDC,
    };
    pub const GRAPH90_SWAP_KNOB_INPUT_MODE: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xDD,
    };
    pub const GRAPH90_WATCH_KNOB_OBJECT: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xDE,
    };
    pub const GRAPH90_UNWATCH_KNOB_OBJECT: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xDF,
    };

    /// Target allocates one of eight CDspObjGroup slots as 0xF1000000 | slot.
    pub const GRAPH90_CREATE_GROUP_OBJECT: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xE0,
    };
    pub const GRAPH90_RELEASE_GROUP_OBJECT: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xE1,
    };
    pub const GRAPH90_SET_GROUP_ENABLED: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xE4,
    };
    /// Target sub_442710 configures CDspObjGroup position and transparency:
    /// ABI `(group, x, y, alpha_parameter)`. It calls vtable+0x2C SetPosition
    /// followed by vtable+0x48 SetAlpha; the fourth argument is not priority.
    pub const GRAPH90_CONFIGURE_GROUP_OBJECT: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xE5,
    };
    pub const GRAPH90_ADD_OBJECT_TO_GROUP: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xE8,
    };
    pub const GRAPH90_REMOVE_OBJECT_FROM_GROUP: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xE9,
    };

    /// Opens one DirectShow movie and returns its duration in milliseconds.
    pub const GRAPH90_OPEN_DIRECTSHOW_MOVIE: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xF0,
    };
    pub const GRAPH90_CLOSE_DIRECTSHOW_MOVIE: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xF1,
    };
    pub const GRAPH90_IS_DIRECTSHOW_MOVIE_PLAYING: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xF2,
    };
    pub const GRAPH90_SET_DIRECTSHOW_MOVIE_VOLUME: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xF3,
    };
    pub const GRAPH90_LOAD_BURIKO_MOVIE_RESOURCE: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xF4,
    };
    pub const GRAPH90_RELEASE_BURIKO_MOVIE_RESOURCE: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xF5,
    };
    pub const GRAPH90_DECODE_BURIKO_MOVIE_FRAME: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xF6,
    };
    pub const GRAPH90_ATTACH_BURIKO_MOVIE_RESOURCE: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xF7,
    };
    pub const GRAPH90_CLEAR_SPRITE_TARGETS: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xF8,
    };
    pub const GRAPH90_REGISTER_SPRITE_TARGET: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xFA,
    };
    pub const GRAPH90_UNREGISTER_SPRITE_TARGET: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xFB,
    };
    pub const GRAPH90_HIT_TEST_SPRITE_TARGETS: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xFC,
    };
    pub const GRAPH90_GET_SPRITE_TARGET_STATE: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0xFD,
    };

    /// Copies a sized BP payload into the process-global two-key graph cache.
    pub const GRAPH91_CACHE_BINARY_RESOURCE: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x03,
    };
    pub const GRAPH91_SET_GLOBAL_DISPLAY_OFFSET: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x06,
    };
    pub const GRAPH91_SET_SCRIPT_BITMAP_CONTEXT_BINDING_ENABLED: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x0B,
    };
    pub const GRAPH91_SET_GLYPH_COVERAGE_MODE: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x0C,
    };
    pub const GRAPH91_SET_FONT_PITCH_DETECTION_ENABLED: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x0D,
    };
    pub const GRAPH91_REGISTER_NAMED_FONT_TRANSFORM: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x0E,
    };
    pub const GRAPH91_CONFIGURE_NATIVE_FONT: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x0F,
    };
    pub const GRAPH91_GENERATE_AFFINE_DISPLACEMENT_MAP: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x10,
    };
    pub const GRAPH91_GENERATE_RANDOM_DISPLACEMENT_MAP: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x11,
    };
    pub const GRAPH91_GENERATE_RIPPLE_DISPLACEMENT_MAP: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x12,
    };
    pub const GRAPH91_GENERATE_PERSPECTIVE_BEND_MAP: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x13,
    };
    pub const GRAPH91_GENERATE_CURVATURE_DISPLACEMENT_MAP: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x14,
    };
    pub const GRAPH91_GENERATE_RADIAL_LENS_DISPLACEMENT_MAP: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x15,
    };
    pub const GRAPH91_GENERATE_SINE_DISPLACEMENT_MAP: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x16,
    };
    pub const GRAPH91_GENERATE_RADIAL_WARP_DISPLACEMENT_MAP: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x17,
    };
    pub const GRAPH91_COMPOSITE_BITMAP_RECT_ALPHA: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x18,
    };
    pub const GRAPH91_COMPOSITE_BITMAP_RECT_CONVERTED: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x19,
    };
    pub const GRAPH91_REPLACE_BITMAP_RGB_PRESERVE_ALPHA: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x1A,
    };
    pub const GRAPH91_CONCENTRATE_BITMAP: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x1B,
    };
    pub const GRAPH91_SCALE_TRUE_COLOR_BITMAP: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x1C,
    };
    pub const GRAPH91_PROCESS_BITMAP: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x1D,
    };
    pub const GRAPH91_APPLY_GRAYSCALE_MASK: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x1E,
    };
    pub const GRAPH91_CLONE_BITMAP: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x1F,
    };

    /// Target CDspObj+0x0C suppression gate, tested inversely by IsDrawable.
    pub const GRAPH91_SET_OBJECT_SUPPRESSED: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x31,
    };
    pub const GRAPH91_SET_OBJECT_FIXED_POSITION: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x33,
    };
    pub const GRAPH91_SET_OBJECT_SECONDARY_VECTOR: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x36,
    };
    pub const GRAPH91_SET_OBJECT_PRIMARY_VECTOR: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x37,
    };
    pub const GRAPH91_GET_OBJECT_PROPERTY: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x38,
    };
    pub const GRAPH91_GET_OBJECT_COMPOSITE_POSITION: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x3D,
    };
    pub const GRAPH91_ATTACH_CHILD_OBJECT: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x3E,
    };
    pub const GRAPH91_DETACH_CHILD_OBJECT: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x3F,
    };
    pub const GRAPH91_INITIALIZE_MULTILAYER_BACKGROUND: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x40,
    };
    pub const GRAPH91_SELECT_MULTILAYER_LAYER: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x41,
    };
    pub const GRAPH91_SET_MULTILAYER_LAYER_ENABLED: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x42,
    };
    pub const GRAPH91_SET_MULTILAYER_LAYER_POSITION: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x43,
    };
    pub const GRAPH91_SET_MULTILAYER_LAYER_BLEND_MODE: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x44,
    };
    pub const GRAPH91_SET_MULTILAYER_LAYER_BLEND_PARAMETER: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x45,
    };
    pub const GRAPH91_SET_MULTILAYER_LAYER_BITMAP: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x46,
    };
    pub const GRAPH91_SET_MULTILAYER_LAYER_TRANSFORM: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x47,
    };
    pub const GRAPH91_SET_MULTILAYER_LAYER_AUXILIARY_PAIR: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x48,
    };
    pub const GRAPH91_SET_MULTILAYER_LAYER_SOURCE_VELOCITY: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x49,
    };
    pub const GRAPH91_SET_MULTILAYER_LAYER_TRANSFORM_VELOCITY: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x4A,
    };

    pub const GRAPH91_SET_SPRITE_RELATION: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x55,
    };
    pub const GRAPH91_CREATE_EFFECTOR: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x60,
    };
    pub const GRAPH91_RELEASE_EFFECTOR: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x61,
    };
    pub const GRAPH91_SET_EFFECTOR_ENABLED: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x64,
    };
    pub const GRAPH91_CONFIGURE_DUAL_VECTOR_EFFECTOR: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x65,
    };
    pub const GRAPH91_CONFIGURE_GRADIENT_EFFECTOR: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x66,
    };
    pub const GRAPH91_CONFIGURE_RIPPLE_EFFECTOR: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x67,
    };
    pub const GRAPH91_CONFIGURE_TRANSFORM_EFFECTOR: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x68,
    };
    pub const GRAPH91_CONFIGURE_SURFACE_EFFECTOR: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x69,
    };
    pub const GRAPH91_CREATE_LANDSCAPE: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x70,
    };
    pub const GRAPH91_RELEASE_LANDSCAPE: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x71,
    };
    pub const GRAPH91_HIT_TEST_LANDSCAPE: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x73,
    };
    pub const GRAPH91_SET_LANDSCAPE_ENABLED: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x74,
    };
    pub const GRAPH91_CONFIGURE_LANDSCAPE_OBJECT: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x75,
    };
    pub const GRAPH91_SET_LANDSCAPE_CELL_SILHOUETTE: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x76,
    };
    pub const GRAPH91_CONFIGURE_LANDSCAPE_PARTS: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x78,
    };
    pub const GRAPH91_CONFIGURE_LANDSCAPE_MAP: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x79,
    };
    pub const GRAPH91_CONFIGURE_LANDSCAPE_GUIDES: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x7A,
    };
    pub const GRAPH91_SET_LANDSCAPE_CELL_GUIDES: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x7B,
    };
    pub const GRAPH91_COPY_LANDSCAPE_PART: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x7C,
    };
    pub const GRAPH91_SET_LANDSCAPE_CELL_COLUMN: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x7D,
    };
    pub const GRAPH91_GET_LANDSCAPE_CELL_VALUE: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x7E,
    };
    pub const GRAPH91_COPY_LANDSCAPE_CELL_IMAGE: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x7F,
    };

    pub const GRAPH91_CONFIGURE_WINDOW_FONT: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x88,
    };
    pub const GRAPH91_SET_WINDOW_LINE_SPACING: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x89,
    };
    pub const GRAPH91_SET_WINDOW_MESSAGE_VARIANT: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x8A,
    };
    pub const GRAPH91_SET_WINDOW_TEXT_LAYOUT_MODE: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x8B,
    };
    pub const GRAPH91_SET_WINDOW_TEXT_CURSOR: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x8C,
    };
    pub const GRAPH91_GET_WINDOW_TEXT_CURSOR: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x8D,
    };
    pub const GRAPH91_IS_WINDOW_TEXT_CURSOR_AT_BOUNDARY: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x8E,
    };
    pub const GRAPH91_START_EXTENDED_MESSAGE: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x90,
    };
    pub const GRAPH91_RENDER_WINDOW_TEXT: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x91,
    };
    pub const GRAPH91_START_EXTENDED_MESSAGE_WITH_OPTION: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x92,
    };
    pub const GRAPH91_RENDER_WINDOW_TEXT_WITH_STYLE_MODE: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x93,
    };
    pub const GRAPH91_UPDATE_TEXT_SUBSTITUTION: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x94,
    };
    pub const GRAPH91_COUNT_TEXT_SUBSTITUTION_MATCHES: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x95,
    };
    pub const GRAPH91_REGISTER_TEXT_SUBSTITUTION_RECORDS: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x96,
    };
    pub const GRAPH91_SET_PERSISTENT_TEXT_STYLE: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x97,
    };
    pub const GRAPH91_CONFIGURE_TEXT_LAYOUT_GLOBALS: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x98,
    };
    pub const GRAPH91_SET_TEXT_SCALE_DIVISOR: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x99,
    };
    pub const GRAPH91_SET_TEXT_GLOBAL_PROPERTY: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x9A,
    };
    pub const GRAPH91_MEASURE_TEXT: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x9B,
    };
    pub const GRAPH91_DRAW_TEXT: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x9C,
    };
    pub const GRAPH91_DRAW_TEXT_WITH_STYLE_MODE: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x9D,
    };
    pub const GRAPH91_EXTRACT_TEXT_LABELS: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x9E,
    };
    pub const GRAPH91_STRIP_TEXT_MARKUP: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0x9F,
    };

    pub const GRAPH91_CREATE_EXTENDED_ICON_INPUT_PROCESSOR: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0xB8,
    };
    pub const GRAPH91_CONFIGURE_EXTENDED_ICON_INPUT_PROCESSOR: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0xBA,
    };
    pub const GRAPH91_SET_EXTENDED_ICON_INPUT_ITEM_STATE: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0xBB,
    };
    pub const GRAPH91_REGISTER_KEY_ASSIGNMENT_TABLE: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0xBF,
    };
    pub const GRAPH91_GET_ACTIVE_KNOB_HANDLE: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0xDB,
    };
    pub const GRAPH91_OPEN_DIRECTSHOW_MOVIE: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0xF0,
    };
    pub const GRAPH91_START_DIRECTSHOW_MOVIE: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0xF1,
    };
    pub const GRAPH91_CLOSE_DIRECTSHOW_MOVIE: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0xF2,
    };
    pub const GRAPH91_SET_DIRECTSHOW_MOVIE_PAUSED: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0xF3,
    };
    pub const GRAPH91_CREATE_FLASH_CONTROL: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0xF4,
    };
    pub const GRAPH91_START_FLASH_CONTROL: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0xF5,
    };
    pub const GRAPH91_CAPTURE_AND_RELEASE_FLASH_CONTROL: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0xF6,
    };
    pub const GRAPH91_GET_MOVIE_POSITION: NativeOpcode = NativeOpcode {
        group: 0x91,
        id: 0xF7,
    };

    /// Target `sub_440C80`: render the second BP argument (graph/display
    /// object) into the first BP argument (destination bitmap).
    pub const GRAPH_RENDER_OBJECT_TO_BITMAP: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x83,
    };
    /// Target `sub_440780`: bind/copy the fourth BP argument bitmap backing to
    /// the first BP argument surface. The two middle parameter meanings remain
    /// unrecovered.
    pub const GRAPH_BIND_BITMAP_TO_SURFACE: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x86,
    };
    pub const GRAPH92_CONFIGURE_COMPACT_WAVE_TABLE: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0x00,
    };
    pub const GRAPH92_CONFIGURE_WAVE_TABLE: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0x01,
    };
    pub const GRAPH92_GENERATE_RADIAL_VECTOR_MAP: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0x10,
    };
    pub const GRAPH92_GENERATE_AXIS_VECTOR_MAP: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0x11,
    };
    pub const GRAPH92_SET_BITMAP_AUXILIARY_PAIR: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0x12,
    };
    pub const GRAPH92_REPLACE_BITMAP_COLOR: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0x13,
    };
    pub const GRAPH92_PRELOAD_BITMAP_RESOURCE: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0x14,
    };
    pub const GRAPH92_CANCEL_PENDING_BITMAP_PRELOADS: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0x15,
    };
    pub const GRAPH92_GET_BITMAP_AUXILIARY_PAIR: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0x16,
    };
    pub const GRAPH92_READ_BITMAP_PIXEL_VALUE: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0x17,
    };
    /// Target `sub_408320`: create/update the first BP argument as a format-3
    /// descriptor using the second BP argument as the format-1/2 source.
    pub const GRAPH_CONVERT_BITMAP_TO_ALPHA_DESCRIPTOR: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0x18,
    };
    pub const GRAPH92_INVERT_ALPHA_BITMAP: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0x19,
    };
    pub const GRAPH92_COMPOSE_BITMAP_ALPHA_AT_OFFSET: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0x1A,
    };
    pub const GRAPH92_DRAW_BITMAP_TEXT_MEASURE: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0x1C,
    };
    pub const GRAPH92_DRAW_WRAPPED_BITMAP_TEXT: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0x1D,
    };
    pub const GRAPH92_DRAW_BITMAP_TEXT: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0x1E,
    };
    pub const GRAPH92_LOAD_EXTERNAL_BMP: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0x1F,
    };

    /// Configures target message-input scope mode/value globals used by the
    /// CProcDspMsg constructor.
    pub const GRAPH_SET_MESSAGE_INPUT_SCOPE: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x91,
    };
    /// Controls the CProcDspMsg host-notify path (`dword_565BAC`).
    pub const GRAPH_SET_MESSAGE_INPUT_FILTER: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x92,
    };
    pub const GRAPH_SET_GLYPH_REVEAL_DELAY: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x94,
    };
    pub const GRAPH_SET_TEXT_REVEAL_ANIMATION: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x95,
    };
    pub const GRAPH_SET_TEXT_SETTLE_ANIMATION: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x96,
    };
    pub const GRAPH_SET_TEXT_AUTO_ADVANCE: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x97,
    };
    pub const GRAPH_SET_MESSAGE_START_DELAY: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x9B,
    };
    pub const GRAPH_SET_MESSAGE_INPUT_FORCES_COMPLETION: NativeOpcode = NativeOpcode {
        group: 0x90,
        id: 0x9F,
    };
    /// Compatibility aliases retained for older runtime match arms.
    pub const GRAPH_SET_MESSAGE_VARIANT: NativeOpcode = GRAPH91_SET_WINDOW_MESSAGE_VARIANT;
    pub const GRAPH_SET_TEXT_EXTENT_FONT_STATE: NativeOpcode =
        GRAPH91_CONFIGURE_TEXT_LAYOUT_GLOBALS;
    pub const GRAPH92_SET_TEXT_OBJECT_VALUE: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0x88,
    };
    pub const GRAPH92_COMPOSITE_RESOURCE_INTO_TEXT_OBJECT: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0x89,
    };
    pub const GRAPH92_SET_TEXT_OBJECT_AUXILIARY_VALUE: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0x8A,
    };
    pub const GRAPH92_SET_TEXT_OBJECT_UPDATE_FLAG: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0x8C,
    };
    pub const GRAPH92_APPLY_EFFECT_RESOURCE: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0x8D,
    };
    pub const GRAPH92_RESET_TEXT_OBJECT: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0x8E,
    };
    pub const GRAPH92_START_STYLED_MESSAGE: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0x90,
    };
    pub const GRAPH92_DRAW_FORMATTED_TEXT: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0x91,
    };
    pub const GRAPH92_CONFIGURE_TEXT_BITMAP_SLOT: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0x98,
    };
    pub const GRAPH92_GET_TEXT_OUTPUT_PAIR: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0x9B,
    };
    pub const GRAPH92_RENDER_TEXT: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0x9C,
    };
    pub const GRAPH92_CONFIGURE_FONT_OVERRIDE: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0x9D,
    };
    pub const GRAPH92_DRAIN_TEXT_FRAGMENT_RECORDS: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0x9E,
    };
    pub const GRAPH92_SET_TEXT_RENDER_OVERRIDE: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0x9F,
    };
    /// Opens one archive-backed global DirectShow movie and returns its duration.
    pub const GRAPH92_OPEN_GLOBAL_DIRECTSHOW_MOVIE: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0xF0,
    };
    /// Installs DCProcLoadBurikoMV/DCProcLoadBMVHeader and writes handle plus metadata outputs.
    pub const GRAPH92_LOAD_BURIKO_MOVIE_RESOURCE: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0xF1,
    };
    /// Creates an archive-backed DirectShow renderer in one bitmap slot.
    pub const GRAPH92_OPEN_BITMAP_DIRECTSHOW_MOVIE: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0xF2,
    };
    /// Seeks one bitmap-backed DirectShow renderer in milliseconds.
    pub const GRAPH92_SEEK_BITMAP_DIRECTSHOW_MOVIE: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0xF4,
    };
    /// Writes one bitmap-backed DirectShow renderer position in milliseconds.
    pub const GRAPH92_GET_BITMAP_DIRECTSHOW_MOVIE_POSITION: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0xF5,
    };
    /// Sets one bitmap-backed DirectShow renderer volume in the target 0..=128 scale.
    pub const GRAPH92_SET_BITMAP_DIRECTSHOW_MOVIE_VOLUME: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0xF6,
    };

    /// Returns the target fixed software-audio channel count (20).
    pub const SOUND_QUERY_CHANNEL_COUNT: NativeOpcode = NativeOpcode {
        group: 0xA0,
        id: 0x00,
    };
    pub const SOUND_SET_BGM_PRIMARY_VOLUME: NativeOpcode = NativeOpcode {
        group: 0xA0,
        id: 0x08,
    };
    pub const SOUND_SET_SE_PRIMARY_VOLUME: NativeOpcode = NativeOpcode {
        group: 0xA0,
        id: 0x09,
    };
    pub const SOUND_LOAD_BGM_FILE: NativeOpcode = NativeOpcode {
        group: 0xA0,
        id: 0x10,
    };
    pub const SOUND_LOAD_BGM_ARCHIVE: NativeOpcode = NativeOpcode {
        group: 0xA0,
        id: 0x11,
    };
    pub const SOUND_LOAD_BGM_PAIR: NativeOpcode = NativeOpcode {
        group: 0xA0,
        id: 0x12,
    };
    pub const SOUND_CONTROL_BGM: NativeOpcode = NativeOpcode {
        group: 0xA0,
        id: 0x14,
    };
    pub const SOUND_QUERY_BGM_STATE: NativeOpcode = NativeOpcode {
        group: 0xA0,
        id: 0x15,
    };
    pub const SOUND_SET_BGM_VOLUME: NativeOpcode = NativeOpcode {
        group: 0xA0,
        id: 0x16,
    };
    pub const SOUND_SET_BGM_PAN: NativeOpcode = NativeOpcode {
        group: 0xA0,
        id: 0x17,
    };
    pub const SOUND_FADE_BGM_TO_FULL: NativeOpcode = NativeOpcode {
        group: 0xA0,
        id: 0x18,
    };
    pub const SOUND_FADE_BGM_TO_SILENCE: NativeOpcode = NativeOpcode {
        group: 0xA0,
        id: 0x19,
    };
    pub const SOUND_SET_BGM_SECONDARY_VOLUME: NativeOpcode = NativeOpcode {
        group: 0xA0,
        id: 0x1C,
    };
    pub const SOUND_LOAD_SE: NativeOpcode = NativeOpcode {
        group: 0xA0,
        id: 0x20,
    };
    pub const SOUND_LOAD_SE_SCALED: NativeOpcode = NativeOpcode {
        group: 0xA0,
        id: 0x21,
    };
    pub const SOUND_RELEASE_SE: NativeOpcode = NativeOpcode {
        group: 0xA0,
        id: 0x22,
    };
    pub const SOUND_LOAD_SE_DOUBLE_RATE: NativeOpcode = NativeOpcode {
        group: 0xA0,
        id: 0x23,
    };
    pub const SOUND_PLAY_SE: NativeOpcode = NativeOpcode {
        group: 0xA0,
        id: 0x24,
    };
    pub const SOUND_STOP_SE: NativeOpcode = NativeOpcode {
        group: 0xA0,
        id: 0x25,
    };
    pub const SOUND_FADE_SE_TO_SILENCE: NativeOpcode = NativeOpcode {
        group: 0xA0,
        id: 0x26,
    };
    pub const SOUND_LOAD_SE_CUSTOM_RATE: NativeOpcode = NativeOpcode {
        group: 0xA0,
        id: 0x27,
    };
    pub const SOUND_REGISTER_SE_MEMORY: NativeOpcode = NativeOpcode {
        group: 0xA0,
        id: 0x28,
    };
    pub const SOUND_SET_SE_SECONDARY_VOLUME: NativeOpcode = NativeOpcode {
        group: 0xA0,
        id: 0x2C,
    };
    pub const SOUND_GET_SE_POSITION: NativeOpcode = NativeOpcode {
        group: 0xA0,
        id: 0x2F,
    };
    pub const SOUND_OPEN_CD_AUDIO: NativeOpcode = NativeOpcode {
        group: 0xA0,
        id: 0x80,
    };
    pub const SOUND_CLOSE_CD_AUDIO: NativeOpcode = NativeOpcode {
        group: 0xA0,
        id: 0x81,
    };
    pub const SOUND_PLAY_CD_TRACK: NativeOpcode = NativeOpcode {
        group: 0xA0,
        id: 0x84,
    };
    pub const SOUND_STOP_CD_AUDIO: NativeOpcode = NativeOpcode {
        group: 0xA0,
        id: 0x85,
    };
    pub const SOUND_QUERY_CD_MODE: NativeOpcode = NativeOpcode {
        group: 0xA0,
        id: 0x86,
    };
    pub const SOUND_PLAY_WAVE_ASYNC: NativeOpcode = NativeOpcode {
        group: 0xA0,
        id: 0xC0,
    };

    // Target SystemB0 user/window/input/font handlers.
    pub const USER_DRAW_BITMAP_TO_WINDOW: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x00,
    };
    pub const USER_CENTER_MAIN_WINDOW: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x02,
    };
    pub const USER_SET_MAIN_WINDOW_POSITION: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x03,
    };
    pub const USER_BIND_CURSOR_OBJECT: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x04,
    };
    pub const USER_SET_CURSOR_IDLE_TIMEOUT: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x05,
    };
    pub const USER_QUERY_CURSOR_VISIBLE: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x06,
    };
    pub const USER_SHAKE_SCREEN: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x08,
    };
    pub const USER_CREATE_DEBUG_WINDOW: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x10,
    };
    pub const USER_CLOSE_DEBUG_WINDOW: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x11,
    };
    pub const USER_SET_DEBUG_WINDOW_VISIBLE: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x14,
    };
    pub const USER_SET_DEBUG_WINDOW_TITLE: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x15,
    };
    pub const USER_MOVE_DEBUG_WINDOW: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x16,
    };
    pub const USER_GET_DEBUG_WINDOW_POSITION: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x17,
    };
    pub const USER_CLEAR_DEBUG_WINDOW: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x18,
    };
    pub const USER_DRAW_DEBUG_BITMAP: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x19,
    };
    pub const USER_DRAW_DEBUG_TEXT: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x1A,
    };
    pub const USER_SET_DEBUG_WINDOW_CLOSE_MESSAGE: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x1C,
    };
    pub const USER_CREATE_EDIT_CONTROL: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x20,
    };
    pub const USER_DESTROY_EDIT_CONTROL: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x21,
    };
    pub const USER_SET_EDIT_FONT_SCALE: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x22,
    };
    pub const USER_QUERY_EDIT_ACTIVE: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x23,
    };
    pub const USER_SET_EDIT_VISIBLE: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x24,
    };
    pub const USER_SET_EDIT_COLOR: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x25,
    };
    pub const USER_SET_EDIT_TEXT: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x26,
    };
    pub const USER_GET_EDIT_TEXT: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x27,
    };
    pub const USER_SET_EDIT_HIDE_ON_ENTER: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x28,
    };
    pub const USER_SET_EDIT_PRINTABLE_INPUT: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x29,
    };
    pub const USER_SHOW_MESSAGE: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x80,
    };
    pub const USER_SHOW_YES_NO_MESSAGE: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x81,
    };
    pub const USER_SHOW_TYPED_MESSAGE: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x82,
    };
    pub const USER_SET_MESSAGE_TITLE: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x83,
    };
    pub const USER_SHOW_INPUT_DIALOG: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x84,
    };
    pub const USER_SHOW_MULTI_FIELD_DIALOG: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x85,
    };
    pub const USER_SHOW_SELECTION_DIALOG: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x86,
    };
    pub const USER_SHOW_EXTENDED_DIALOG: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x87,
    };
    pub const USER_SHOW_PATH_DIALOG: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x8C,
    };
    pub const USER_SHOW_SIX_FIELD_DIALOG: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0x8F,
    };
    pub const USER_CREATE_MODELESS_DIALOG: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0xA0,
    };
    pub const USER_CLOSE_MODELESS_DIALOG: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0xA1,
    };
    pub const USER_SET_MODELESS_DIALOG_VISIBLE: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0xA2,
    };
    pub const USER_POLL_MODELESS_DIALOG: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0xA3,
    };
    pub const USER_INTERN_FONT_NAME: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0xC0,
    };
    pub const USER_INTERN_FONT_NAME_WITH_OPTION: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0xC1,
    };
    pub const USER_REGISTER_FONT_RESOURCE: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0xC2,
    };
    pub const USER_REGISTER_ARCHIVE_FONT_RESOURCE: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0xC3,
    };
    pub const USER_QUERY_FONT_AVAILABLE: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0xC4,
    };
    pub const USER_QUERY_FONT_CAPABILITY: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0xC6,
    };
    pub const USER_SET_FONT_ALIAS: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0xC7,
    };
    pub const USER_SET_DESKTOP_WALLPAPER: NativeOpcode = NativeOpcode {
        group: 0xB0,
        id: 0xF0,
    };

    // Target SystemC0 particle, rain, spline, and BWEF handlers.
    pub const USER2_CREATE_PARTICLE_SCREEN: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x00,
    };
    pub const USER2_RELEASE_PARTICLE_SCREEN: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x01,
    };
    pub const USER2_SET_PARTICLE_SCREEN_ENABLED: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x04,
    };
    pub const USER2_CONFIGURE_PARTICLE_SCREEN_DISPLAY: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x05,
    };
    pub const USER2_CONFIGURE_PARTICLE_FRAME_TABLES: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x06,
    };
    pub const USER2_COMMIT_PARTICLE_SCREEN: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x08,
    };
    pub const USER2_SET_PARTICLE_AUTO_UPDATE: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x09,
    };
    pub const USER2_SET_PARTICLE_CAPACITY: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x0A,
    };
    pub const USER2_CONFIGURE_PARTICLE_EMITTER_TRANSFORM: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x0B,
    };
    pub const USER2_SET_PARTICLE_EMISSION_PERCENT: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x0C,
    };
    pub const USER2_ADVANCE_PARTICLE_SCREEN: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x0D,
    };
    pub const USER2_RESET_PARTICLE_SCREEN: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x0F,
    };
    pub const USER2_CONFIGURE_PARTICLE_EMITTER: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x10,
    };
    pub const USER2_CONFIGURE_PARTICLE_ANIMATION_BANK: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x18,
    };
    pub const USER2_LOAD_PARTICLE_ANIMATION_FRAMES: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x1A,
    };
    pub const USER2_COMMIT_PARTICLE_ANIMATION_FRAMES: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x1B,
    };
    pub const USER2_SET_PARTICLE_INTERPOLATION_MODE: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x1F,
    };
    pub const USER2_SET_PARTICLE_STYLE0: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x20,
    };
    pub const USER2_DEFINE_PARTICLE_STYLE0: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x24,
    };
    pub const USER2_CONFIGURE_PARTICLE_STYLE0: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x25,
    };
    pub const USER2_SET_PARTICLE_STYLE1: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x28,
    };
    pub const USER2_CONFIGURE_PARTICLE_STYLE1: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x29,
    };
    pub const USER2_DEFINE_PARTICLE_STYLE1: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x2C,
    };
    pub const USER2_CONFIGURE_PARTICLE_ADVANCED_STYLE: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x2D,
    };
    pub const USER2_CREATE_RAIN_SCREEN: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x40,
    };
    pub const USER2_RELEASE_RAIN_SCREEN: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x41,
    };
    pub const USER2_INITIALIZE_RAIN_SCREEN: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x42,
    };
    pub const USER2_SET_RAIN_TEXTURE: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x43,
    };
    pub const USER2_SET_RAIN_ENABLED: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x44,
    };
    pub const USER2_CONFIGURE_RAIN_DISPLAY: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x45,
    };
    pub const USER2_SET_RAIN_VOLUME_BOUNDS: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x46,
    };
    pub const USER2_SET_RAIN_DROP_WIDTH: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x47,
    };
    pub const USER2_SET_RAIN_DROP_HEIGHT: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x48,
    };
    pub const USER2_SET_RAIN_COLOR: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x49,
    };
    pub const USER2_SET_RAIN_DENSITY: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x4A,
    };
    pub const USER2_SET_RAIN_SPEED: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x4B,
    };
    pub const USER2_SET_RAIN_ORIGIN: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x4C,
    };
    pub const USER2_SET_RAIN_DIRECTION: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x4D,
    };
    pub const USER2_SET_RAIN_LENGTH: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x4E,
    };
    pub const USER2_SET_RAIN_STEP: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0x4F,
    };
    pub const USER2_CREATE_SPLINE: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0xC0,
    };
    pub const USER2_RELEASE_SPLINE: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0xC1,
    };
    pub const USER2_CONFIGURE_SPLINE: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0xC2,
    };
    pub const USER2_SAMPLE_SPLINE: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0xC3,
    };
    pub const USER2_LOAD_BWEF_TABLE: NativeOpcode = NativeOpcode {
        group: 0xC0,
        id: 0xF0,
    };

    /// Target helper 0x00434440 is explicitly used by Graph92:0x97 and
    /// stores six persistent text-style fields.
    pub const GRAPH_SET_PERSISTENT_TEXT_STYLE: NativeOpcode = NativeOpcode {
        group: 0x92,
        id: 0x97,
    };
}

/// Strength of target-engine evidence for a native handler.
///
/// Handler names from external decompilers never raise this level by
/// themselves. `TargetConfirmed` requires target dispatch/decompilation, ABI,
/// call-site, or runtime evidence that closes the selector and core contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeRecoveryLevel {
    TargetConfirmed,
    TargetInferred,
    AbiOnly,
    CandidateNameOnly,
    Unrecovered,
}

/// Fidelity of the current Rust implementation for a native handler.
///
/// This is deliberately independent from recovery level. A handler may have a
/// target-confirmed contract while the portable implementation is still only
/// partial or a stub.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeImplementationLevel {
    PortableEquivalent,
    Partial,
    CompatibilityFallback,
    Stub,
    NotAudited,
}

/// Scheduling effect recovered for a native opcode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeSchedulingEffect {
    /// The call returns synchronously and execution may continue.
    Continue,
    /// The current coroutine voluntarily ends its scheduler slice.
    Yield,
    /// Another coroutine is made runnable and the caller ends its slice.
    SwitchCoroutine,
    /// The call installs a cooperative native procedure.
    WaitProcedure,
    /// The call terminates the active interpreter/thread; async tasks are not
    /// resumed after native status 6.
    TerminateInterpreter,
}

/// Human-readable parameter documentation for an opcode whose ABI has been
/// recovered beyond the raw argument count.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativeParameterSpec {
    pub name: &'static str,
    pub kind: &'static str,
    pub description: &'static str,
}

/// Runtime and documentation metadata for a native opcode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativeOpcodeSpec {
    pub opcode: NativeOpcode,
    pub symbol: &'static str,
    pub parameters: &'static [NativeParameterSpec],
    pub returns: &'static str,
    pub scheduling: NativeSchedulingEffect,
    pub notes: &'static str,
}

const NO_PARAMETERS: &[NativeParameterSpec] = &[];
const RNG_SEED_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "seed",
    kind: "u32 seed",
    description: "Passed directly to the target Microsoft CRT srand implementation.",
}];
const RAND_MAX_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "max_exclusive",
    kind: "i32",
    description: "Positive modulus bound; non-positive values return zero without calling rand.",
}];
const PERFORMANCE_COUNTER_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "counter_out",
    kind: "BP pointer to u64",
    description: "Receives QueryPerformanceCounter / frequency scaled to nanoseconds.",
}];
const PERFORMANCE_PROFILING_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "enabled",
    kind: "bool/i32",
    description: "Stored in the target profiling gate; nonzero also resets all accumulated counters.",
}];
const PERFORMANCE_METRIC_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "output_ptr",
        kind: "BP pointer to i32",
        description: "Receives the selected target profiling metric.",
    },
    NativeParameterSpec {
        name: "metric_id",
        kind: "i32 enum 0..=3",
        description: "0=count, 1=average microseconds, 2=throughput ratio, 3=dispersion ratio; invalid selectors write zero.",
    },
];
const GRAPHICS_CAPABILITY_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "destination",
    kind: "BP pointer to 64-byte record",
    description: "Receives the target 16-DWORD graphics capability cache verbatim.",
}];
const SYSTEM_TIME_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "destination",
    kind: "BP pointer to SYSTEMTIME",
    description: "Receives eight little-endian u16 fields from Win32 GetLocalTime.",
}];
const INPUT_CONFIGURATION_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "enabled",
    kind: "bool/i32",
    description: "Stored in target dword_506A44 before all per-input state records are cleared.",
}];
const KEY_STATE_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "descriptor",
    kind: "i32 target logical input descriptor",
    description: "Mapped through the target key/button mapping and returned as zero or one from the async-key high bit.",
}];
const INPUT_DESCRIPTOR_LIST_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "descriptors",
    kind: "pointer to linked/zero-terminated descriptor record",
    description: "The handler traverses descriptor identifiers and sums each target current-count field.",
}];
const MOUSE_BUTTON_MAPPING_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "mode",
    kind: "i32 enum 0..=1",
    description: "Mode 1 swaps logical descriptors 1 and 2; values above 1 are rejected.",
}];
const ALLOC_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "size",
    kind: "u32 byte count",
    description: "Requested target VM memory-class block size; allocation failure raises the native VM error path.",
}];
const FREE_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "pointer",
    kind: "opaque BP memory pointer",
    description: "Null succeeds; a non-null value must identify a currently allocated target memory-class slot.",
}];
const COUNT_FILES_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "pattern",
        kind: "Shift-JIS path pattern",
        description: "Win32 FindFirstFileA pattern.",
    },
    NativeParameterSpec {
        name: "recursive",
        kind: "bool/i32",
        description: "Nonzero recursively scans matching subdirectories.",
    },
];
const ENUMERATE_FILES_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "BP byte-buffer pointer or null",
        description: "Receives packed NUL-terminated names; null requests the required byte count.",
    },
    NativeParameterSpec {
        name: "capacity",
        kind: "i32 byte count",
        description: "Available destination bytes; insufficient capacity returns -1.",
    },
    NativeParameterSpec {
        name: "pattern",
        kind: "Shift-JIS path pattern",
        description: "Pattern passed to FindFirstFileA.",
    },
    NativeParameterSpec {
        name: "recursive",
        kind: "bool/i32",
        description: "Nonzero descends into subdirectories.",
    },
    NativeParameterSpec {
        name: "max_count",
        kind: "u32",
        description: "Maximum entries; zero means unlimited.",
    },
];
const ENUMERATE_DIRECTORIES_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "BP byte-buffer pointer or null",
        description: "Receives packed immediate subdirectory names; null returns required bytes.",
    },
    NativeParameterSpec {
        name: "capacity",
        kind: "u32 byte count",
        description: "Available destination bytes.",
    },
    NativeParameterSpec {
        name: "pattern",
        kind: "Shift-JIS path pattern",
        description: "Pattern passed to FindFirstFileA.",
    },
    NativeParameterSpec {
        name: "max_count",
        kind: "u32",
        description: "Maximum directories; zero means unlimited.",
    },
];
const TWO_PATH_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "Shift-JIS path",
        description: "First BP argument; target passes the second popped string as the Win32 destination.",
    },
    NativeParameterSpec {
        name: "source",
        kind: "Shift-JIS path",
        description: "Second BP argument and first native pop; target passes it as the Win32 source.",
    },
];
const PATH_PARAMETER: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "path",
    kind: "Shift-JIS path",
    description: "Native filesystem path resolved using the target process working-directory rules.",
}];
const SPLIT_PATH_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "drive_out",
        kind: "BP char buffer or null",
        description: "Receives the drive component.",
    },
    NativeParameterSpec {
        name: "directory_out",
        kind: "BP char buffer or null",
        description: "Receives the directory component.",
    },
    NativeParameterSpec {
        name: "filename_out",
        kind: "BP char buffer or null",
        description: "Receives the filename without extension.",
    },
    NativeParameterSpec {
        name: "extension_out",
        kind: "BP char buffer or null",
        description: "Receives the extension including its leading dot.",
    },
    NativeParameterSpec {
        name: "source_path",
        kind: "Shift-JIS path or null",
        description: "Input path; null returns zero without writing outputs.",
    },
];
const SET_ATTRIBUTES_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "path",
        kind: "Shift-JIS path",
        description: "Target file or directory.",
    },
    NativeParameterSpec {
        name: "attributes",
        kind: "u32 Win32 attribute mask",
        description: "Passed directly to SetFileAttributesA.",
    },
];
const READ_FILE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "BP byte-buffer pointer",
        description: "Receives file/resource bytes.",
    },
    NativeParameterSpec {
        name: "archive_or_root",
        kind: "Shift-JIS string or null",
        description: "Optional explicit archive/search root.",
    },
    NativeParameterSpec {
        name: "file",
        kind: "Shift-JIS resource path",
        description: "Resource/file name searched through target roots.",
    },
];
const READ_FILE_RANGE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "BP byte-buffer pointer",
        description: "Receives the requested file bytes.",
    },
    NativeParameterSpec {
        name: "secondary_root",
        kind: "Shift-JIS path or null",
        description: "Optional secondary search root supplied to sub_465C30.",
    },
    NativeParameterSpec {
        name: "file",
        kind: "Shift-JIS resource path",
        description: "File/resource name resolved through target roots.",
    },
    NativeParameterSpec {
        name: "offset",
        kind: "u32 byte offset",
        description: "Start offset; zero with zero length requests the whole file.",
    },
    NativeParameterSpec {
        name: "length",
        kind: "u32 byte count",
        description: "Requested length; zero is valid only for the whole-file form.",
    },
];
const RESOURCE_SEARCH_ENABLE_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "enabled",
    kind: "boolean i32",
    description: "Stored in target dword_506BE0.",
}];
const RESOURCE_SEARCH_PATH_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "path",
    kind: "Shift-JIS path",
    description: "Copied into a newly prepended linked-list node.",
}];
const COMPOSITE_ARCHIVE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "name",
        kind: "Shift-JIS archive name",
        description: "Name assigned to the DCArchiveComplex record.",
    },
    NativeParameterSpec {
        name: "components",
        kind: "pointer to zero-terminated string-pointer array",
        description: "Archive/component names used to construct the composite archive.",
    },
];
const SPECIAL_FOLDER_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "BP char buffer",
        description: "Receives the selected folder path.",
    },
    NativeParameterSpec {
        name: "mode",
        kind: "i32 enum 0..=5",
        description: "Selects Windows, desktop, programs, documents, common-program-files, or ProgramFilesDir.",
    },
];
const FILE_DIALOG_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "initial_directory",
        kind: "Shift-JIS path",
        description: "Initial directory for the native dialog.",
    },
    NativeParameterSpec {
        name: "description",
        kind: "Shift-JIS string",
        description: "Filter description.",
    },
    NativeParameterSpec {
        name: "extension",
        kind: "Shift-JIS extension",
        description: "Filter extension.",
    },
    NativeParameterSpec {
        name: "destination",
        kind: "BP char buffer",
        description: "Receives the selected path.",
    },
    NativeParameterSpec {
        name: "title",
        kind: "Shift-JIS string",
        description: "Dialog title.",
    },
    NativeParameterSpec {
        name: "mode",
        kind: "i32 enum 0..=1",
        description: "Selects open or save dialog.",
    },
];
const REQUIRE_RESOURCE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "file",
        kind: "Shift-JIS resource path",
        description: "Required file searched under configured roots.",
    },
    NativeParameterSpec {
        name: "ignored_context",
        kind: "Shift-JIS string",
        description: "Popped but not used by the target handler.",
    },
    NativeParameterSpec {
        name: "message",
        kind: "Shift-JIS prompt text",
        description: "Displayed in the target retry/quit loop.",
    },
];
const REMOVABLE_ARCHIVE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "archive_name",
        kind: "Shift-JIS marker/archive name",
        description: "Archive marker searched on removable drives.",
    },
    NativeParameterSpec {
        name: "subdirectory",
        kind: "Shift-JIS relative directory",
        description: "Optional directory below a candidate drive root.",
    },
    NativeParameterSpec {
        name: "prompt",
        kind: "Shift-JIS prompt text",
        description: "Retry prompt used by the target host.",
    },
    NativeParameterSpec {
        name: "retry",
        kind: "boolean i32",
        description: "Nonzero enables the target retry loop.",
    },
];
const LOAD_PROGRAM_MODULE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "archive",
        kind: "Shift-JIS archive",
        description: "Archive containing the BP module.",
    },
    NativeParameterSpec {
        name: "file",
        kind: "Shift-JIS BP filename",
        description: "Decoded and appended to the current CThread code region.",
    },
];
const LOAD_PROGRAM_THREAD_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "archive",
        kind: "Shift-JIS archive",
        description: "Archive containing the BP module.",
    },
    NativeParameterSpec {
        name: "file",
        kind: "Shift-JIS BP filename",
        description: "Initial module for the child CThread.",
    },
    NativeParameterSpec {
        name: "operand_slots",
        kind: "i32 count",
        description: "Child operand-stack/slot allocation input.",
    },
    NativeParameterSpec {
        name: "code_bytes",
        kind: "i32 byte count",
        description: "Child CThread code-region capacity.",
    },
    NativeParameterSpec {
        name: "data_bytes",
        kind: "i32 byte count",
        description: "Child CThread data-region capacity.",
    },
];
const THREAD_ID_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "thread_id",
    kind: "i32 CThread identifier",
    description: "Nonzero thread identifier searched recursively from the current root.",
}];

const FREE_PROGRAM_ABI_SLOT_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "legacy_descriptor_slot",
    kind: "ignored ABI slot",
    description: "The recovered ABI descriptor reports one input, but target handler sub_488CD0 performs no pop and removes the current CThread tail module directly.",
}];

const WRITE_FILE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "path",
        kind: "Shift-JIS path",
        description: "Output file path.",
    },
    NativeParameterSpec {
        name: "source",
        kind: "BP byte-buffer pointer",
        description: "Source bytes.",
    },
    NativeParameterSpec {
        name: "length",
        kind: "u32 byte count",
        description: "Exact requested write count.",
    },
];
const DELETE_FILE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "root",
        kind: "Shift-JIS path or null",
        description: "When non-null, target constructs root\\file; otherwise it prefixes the primary global root.",
    },
    NativeParameterSpec {
        name: "file",
        kind: "Shift-JIS relative path",
        description: "File name deleted by DeleteFileA.",
    },
];
const FILE_EXISTS_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "archive",
        kind: "Shift-JIS archive/root",
        description: "Optional archive container or filesystem root passed to sub_4665C0.",
    },
    NativeParameterSpec {
        name: "file",
        kind: "Shift-JIS resource path",
        description: "Resource/file name passed in ECX to sub_4665C0.",
    },
];
const FILE_SIZE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "archive",
        kind: "Shift-JIS archive/root",
        description: "Optional archive container or filesystem root passed in ECX to sub_4662E0.",
    },
    NativeParameterSpec {
        name: "file",
        kind: "Shift-JIS resource path",
        description: "Resource/file name passed on the stack to sub_4662E0.",
    },
];
const CONFIGURED_ROOT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "BP char buffer",
        description: "Receives the selected configured root including its terminating NUL.",
    },
    NativeParameterSpec {
        name: "kind",
        kind: "i32 enum 0..=1",
        description: "0 selects the primary root; 1 selects the secondary root when configured.",
    },
];
const UNRECOVERED_SINGLE_PARAMETER: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "arg0",
    kind: "unrecovered BP value",
    description: "Target ABI proves one input, but neither its type nor meaning is currently proven.",
}];
const SYSTEM_WAIT_STATE_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "value",
    kind: "i32 global system state",
    description: "Stored unchanged in target dword_507688 by sub_4319B0.",
}];
const THREAD_TIMER_DURATION_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "duration_ms",
    kind: "i32 milliseconds",
    description: "Passed to CThread timer setter; the handler then queries whether remaining time is nonzero.",
}];
const INPUT_GATE_PARAMETER: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "value",
    kind: "i32 configuration value",
    description: "Updates input configuration only; it is not itself an input event.",
}];
const INPUT_SCOPE_PARAMETER: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "scope",
    kind: "i32 input scope",
    description: "The target packs this value as `(scope << 16) | 0xFFFF` before registering or querying it.",
}];
const GLYPH_DELAY_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "delay_ms",
    kind: "i32 milliseconds",
    description: "Delay between glyph reveal steps.",
}];
const TEXT_ANIMATION_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "steps",
        kind: "i32 count",
        description: "Number of reveal or settle animation steps.",
    },
    NativeParameterSpec {
        name: "step_delay_ms",
        kind: "i32 milliseconds",
        description: "Delay between animation steps.",
    },
];
const TEXT_ENABLE_DELAY_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "enabled",
        kind: "bool/i32",
        description: "Nonzero enables the text timing feature.",
    },
    NativeParameterSpec {
        name: "delay_ms",
        kind: "i32 milliseconds",
        description: "Delay used when the feature is enabled.",
    },
];
const BOOLEAN_PARAMETER: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "enabled",
    kind: "bool/i32",
    description: "Nonzero enables the feature.",
}];
const GRAPH_TEXT_EXTENT_FONT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "extent_x",
        kind: "i32",
        description: "Horizontal text extent/state value passed to target helper 0x00434340.",
    },
    NativeParameterSpec {
        name: "extent_y",
        kind: "i32",
        description: "Vertical text extent/state value passed to target helper 0x00434340.",
    },
    NativeParameterSpec {
        name: "reserved",
        kind: "i32 unrecovered",
        description: "Third BP argument; target field meaning remains unresolved.",
    },
    NativeParameterSpec {
        name: "font_size",
        kind: "i32",
        description: "Target helper validates the accepted native font-size range.",
    },
    NativeParameterSpec {
        name: "line_height",
        kind: "i32",
        description: "Target helper requires a nonnegative line-height value.",
    },
    NativeParameterSpec {
        name: "mode",
        kind: "i32",
        description: "Final BP argument; exact mode enumeration remains unresolved.",
    },
];
const GRAPH92_TEXT_OBJECT_VALUE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "object",
        kind: "text object handle",
        description: "Object resolved through the target System92 text-object registry.",
    },
    NativeParameterSpec {
        name: "value",
        kind: "i32",
        description: "Scalar stored in the recovered native object field/substructure.",
    },
];
const GRAPH92_TEXT_OBJECT_COMPOSITE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "object",
        kind: "text object handle",
        description: "Destination text object.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "i32",
        description: "Destination X offset.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32",
        description: "Destination Y offset.",
    },
    NativeParameterSpec {
        name: "resource",
        kind: "bitmap/resource handle",
        description: "Source descriptor resolved by the target bitmap manager.",
    },
    NativeParameterSpec {
        name: "parameter_4",
        kind: "i32",
        description: "Native composition parameter forwarded unchanged.",
    },
    NativeParameterSpec {
        name: "parameter_5",
        kind: "i32",
        description: "Native composition parameter forwarded unchanged.",
    },
];
const GRAPH92_RESET_TEXT_OBJECT_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "object",
    kind: "text object handle",
    description: "Object whose cursor, bitmap, state table, and child text records are cleared.",
}];
const GRAPH92_START_STYLED_MESSAGE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "window",
        kind: "CDspObjWindow handle",
        description: "Message window resolved by sub_4406C0.",
    },
    NativeParameterSpec {
        name: "text",
        kind: "script string",
        description: "Message text passed to CProcDspMsgEx/CProcDspMsgExVE.",
    },
    NativeParameterSpec {
        name: "argument_02",
        kind: "i32",
        description: "Recovered constructor/control operand forwarded to sub_491220.",
    },
    NativeParameterSpec {
        name: "argument_03",
        kind: "i32",
        description: "Recovered constructor/control operand forwarded to sub_491220.",
    },
    NativeParameterSpec {
        name: "argument_04",
        kind: "i32",
        description: "Recovered constructor/control operand forwarded to sub_491220.",
    },
    NativeParameterSpec {
        name: "argument_05",
        kind: "i32",
        description: "Recovered constructor/control operand forwarded to sub_491220.",
    },
    NativeParameterSpec {
        name: "style_0",
        kind: "i32",
        description: "Input to the five-DWORD style tuple built by sub_434E30.",
    },
    NativeParameterSpec {
        name: "style_1",
        kind: "i32",
        description: "Input to the five-DWORD style tuple built by sub_434E30.",
    },
    NativeParameterSpec {
        name: "style_2",
        kind: "i32",
        description: "Input to the five-DWORD style tuple built by sub_434E30.",
    },
    NativeParameterSpec {
        name: "style_3",
        kind: "i32",
        description: "Input to the five-DWORD style tuple built by sub_434E30.",
    },
    NativeParameterSpec {
        name: "style_4",
        kind: "i32",
        description: "Input to the five-DWORD style tuple built by sub_434E30.",
    },
    NativeParameterSpec {
        name: "control_0",
        kind: "i32",
        description: "CProcDspMsgEx control operand.",
    },
    NativeParameterSpec {
        name: "control_1",
        kind: "i32",
        description: "CProcDspMsgEx control operand.",
    },
    NativeParameterSpec {
        name: "control_2",
        kind: "i32",
        description: "CProcDspMsgEx control operand.",
    },
    NativeParameterSpec {
        name: "control_3",
        kind: "i32",
        description: "CProcDspMsgEx control operand.",
    },
];
const GRAPH92_DRAW_FORMATTED_TEXT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "object",
        kind: "text object handle",
        description: "Destination text object.",
    },
    NativeParameterSpec {
        name: "text",
        kind: "script string",
        description: "Formatted text rendered directly into the object.",
    },
    NativeParameterSpec {
        name: "argument_02",
        kind: "i32",
        description: "Text-render operand forwarded to sub_42B710.",
    },
    NativeParameterSpec {
        name: "argument_03",
        kind: "i32",
        description: "Text-render operand forwarded to sub_42B710.",
    },
    NativeParameterSpec {
        name: "argument_04",
        kind: "i32",
        description: "Text-render operand forwarded to sub_42B710.",
    },
    NativeParameterSpec {
        name: "argument_05",
        kind: "i32",
        description: "Text-render operand forwarded to sub_42B710.",
    },
    NativeParameterSpec {
        name: "style_0",
        kind: "i32",
        description: "Input to sub_434E30.",
    },
    NativeParameterSpec {
        name: "style_1",
        kind: "i32",
        description: "Input to sub_434E30.",
    },
    NativeParameterSpec {
        name: "style_2",
        kind: "i32",
        description: "Input to sub_434E30.",
    },
    NativeParameterSpec {
        name: "style_3",
        kind: "i32",
        description: "Input to sub_434E30.",
    },
    NativeParameterSpec {
        name: "style_4",
        kind: "i32",
        description: "Input to sub_434E30.",
    },
];
const GRAPH92_CONFIGURE_TEXT_BITMAP_SLOT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "slot",
        kind: "fixed text-bitmap slot",
        description: "Slot validated by sub_432E30.",
    },
    NativeParameterSpec {
        name: "source",
        kind: "bitmap handle or -1",
        description: "Source bitmap; -1 releases the slot.",
    },
    NativeParameterSpec {
        name: "offset_x",
        kind: "i32",
        description: "Crop/translation X operand.",
    },
    NativeParameterSpec {
        name: "offset_y",
        kind: "i32",
        description: "Crop/translation Y operand.",
    },
    NativeParameterSpec {
        name: "width",
        kind: "i32",
        description: "Required nonzero copied width.",
    },
    NativeParameterSpec {
        name: "height",
        kind: "i32",
        description: "Required nonzero copied height.",
    },
];
const GRAPH92_GET_TEXT_OUTPUT_PAIR_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "BP pointer to two DWORDs",
        description: "Receives dword_565D34 and dword_565D38 through the VM pointer bridge.",
    },
    NativeParameterSpec {
        name: "selector",
        kind: "constant 256",
        description: "Only selector accepted by target helper sub_434410.",
    },
];
const GRAPH92_RENDER_TEXT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "target",
        kind: "writable bitmap handle",
        description: "Destination bitmap descriptor validated before native rasterization.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "in/out i32",
        description: "Initial X; target publishes the resulting cursor X through the output pair.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "in/out i32",
        description: "Initial Y and second output-pair component.",
    },
    NativeParameterSpec {
        name: "text",
        kind: "script string",
        description: "Primary text converted by sub_48DF50.",
    },
    NativeParameterSpec {
        name: "packed_rgb",
        kind: "packed 0xRRGGBB",
        description: "Primary text color; zero is a valid black color.",
    },
    NativeParameterSpec {
        name: "argument_05",
        kind: "i32",
        description: "Native render operand forwarded to sub_434BA0.",
    },
    NativeParameterSpec {
        name: "auxiliary_text",
        kind: "nullable script string",
        description: "Second pointer converted by sub_48DF50.",
    },
    NativeParameterSpec {
        name: "secondary_packed_rgb",
        kind: "packed 0xRRGGBB/i32",
        description: "Secondary native text/style color operand.",
    },
    NativeParameterSpec {
        name: "argument_08",
        kind: "i32",
        description: "Native validation/render operand.",
    },
    NativeParameterSpec {
        name: "font_size",
        kind: "i32 pixels",
        description: "Source argument 9; paired with horizontal scale by sub_4035A0.",
    },
    NativeParameterSpec {
        name: "horizontal_scale_percent",
        kind: "i32 percent",
        description: "Source argument 10 paired with font_size by sub_4035A0.",
    },
    NativeParameterSpec {
        name: "argument_11",
        kind: "i32",
        description: "Native render operand; this is not the font-size argument.",
    },
    NativeParameterSpec {
        name: "argument_12",
        kind: "i32",
        description: "Native render operand.",
    },
    NativeParameterSpec {
        name: "character_spacing",
        kind: "i32 pixels",
        description: "Character advance spacing forwarded into the target text layout path.",
    },
    NativeParameterSpec {
        name: "argument_14",
        kind: "i32",
        description: "Native render operand.",
    },
    NativeParameterSpec {
        name: "style_mode",
        kind: "i32 0..2",
        description: "First sub_434E30 style field; zero clears the style block.",
    },
    NativeParameterSpec {
        name: "style_x_percent",
        kind: "i32 0..100",
        description: "Second sub_434E30 style field.",
    },
    NativeParameterSpec {
        name: "style_y_percent",
        kind: "i32 0..100",
        description: "Third sub_434E30 style field.",
    },
    NativeParameterSpec {
        name: "style_color_or_parameter",
        kind: "i32",
        description: "Fourth sub_434E30 style field.",
    },
    NativeParameterSpec {
        name: "style_alpha_or_concentration",
        kind: "i32 0..256",
        description: "Fifth sub_434E30 style field.",
    },
    NativeParameterSpec {
        name: "ignored",
        kind: "i32",
        description: "First native pop; consumed but not forwarded by sub_4867D0.",
    },
];
const GRAPH92_CONFIGURE_FONT_OVERRIDE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "font_name",
        kind: "target string-registry selector",
        description: "Face name resolved through sub_468BB0.",
    },
    NativeParameterSpec {
        name: "height",
        kind: "i32",
        description: "Native font height passed to sub_42EAB0.",
    },
    NativeParameterSpec {
        name: "weight_or_style",
        kind: "i32",
        description: "Native font creation operand.",
    },
    NativeParameterSpec {
        name: "quality_or_charset",
        kind: "i32",
        description: "Native font creation operand.",
    },
    NativeParameterSpec {
        name: "italic",
        kind: "boolean i32",
        description: "Stored in target bItalic after validation.",
    },
];
const GRAPH92_DRAIN_TEXT_FRAGMENT_RECORDS_PARAMETERS: &[NativeParameterSpec] =
    &[NativeParameterSpec {
        name: "destination",
        kind: "BP pointer",
        description: "Receives up to sixteen 128-byte Shift-JIS/x/y records.",
    }];
const GRAPH92_SET_TEXT_RENDER_OVERRIDE_PARAMETERS: &[NativeParameterSpec] =
    &[NativeParameterSpec {
        name: "value",
        kind: "i32",
        description: "Stored in target dword_507650 and read by the text renderer.",
    }];
const GRAPH92_OPEN_GLOBAL_MOVIE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "archive",
        kind: "script string",
        description: "Optional archive/search namespace converted by sub_48DF50.",
    },
    NativeParameterSpec {
        name: "resource",
        kind: "script string",
        description: "Movie resource converted by sub_48DF50 and forwarded to sub_48F270.",
    },
    NativeParameterSpec {
        name: "argument_02",
        kind: "i32",
        description: "Consumed by sub_486B80 but not forwarded to sub_48F270; exact meaning remains unresolved.",
    },
    NativeParameterSpec {
        name: "argument_03",
        kind: "i32",
        description: "Consumed by sub_486B80 but not forwarded to sub_48F270; exact meaning remains unresolved.",
    },
    NativeParameterSpec {
        name: "width",
        kind: "positive i32",
        description: "Validated by sub_486B80 before the global movie is opened.",
    },
    NativeParameterSpec {
        name: "height",
        kind: "positive i32",
        description: "Validated by sub_486B80 before the global movie is opened.",
    },
];
const GRAPH92_LOAD_BURIKO_MOVIE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle_out",
        kind: "BP pointer to DWORD",
        description: "Receives the loaded BF_Movie handle through the VM pointer bridge.",
    },
    NativeParameterSpec {
        name: "metadata_out",
        kind: "BP pointer to five DWORDs",
        description: "Receives the five-word native BF_Movie metadata tuple.",
    },
    NativeParameterSpec {
        name: "archive",
        kind: "script string",
        description: "Optional archive/search namespace.",
    },
    NativeParameterSpec {
        name: "resource",
        kind: "script string",
        description: "Buriko movie resource name.",
    },
    NativeParameterSpec {
        name: "mode",
        kind: "i32",
        description: "Zero selects DCProcLoadBurikoMV; nonzero selects DCProcLoadBMVHeader.",
    },
];
const GRAPH92_OPEN_BITMAP_MOVIE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap slot 0..0x3fff",
        description: "Destination slot receiving the native DCMovieRenderer.",
    },
    NativeParameterSpec {
        name: "archive",
        kind: "script string",
        description: "Optional archive/search namespace.",
    },
    NativeParameterSpec {
        name: "resource",
        kind: "script string",
        description: "DirectShow movie resource.",
    },
    NativeParameterSpec {
        name: "looping",
        kind: "boolean i32",
        description: "Stored in the native renderer loop field.",
    },
    NativeParameterSpec {
        name: "volume",
        kind: "i32 0..=128",
        description: "Validated by sub_44D450 and stored in the renderer.",
    },
];
const GRAPH92_SEEK_BITMAP_MOVIE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap slot",
        description: "Slot whose DCMovieRenderer is sought.",
    },
    NativeParameterSpec {
        name: "position_ms",
        kind: "i32 milliseconds",
        description: "Absolute media position forwarded to sub_44D2C0.",
    },
];
const GRAPH92_GET_BITMAP_MOVIE_POSITION_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "position_out",
        kind: "BP pointer to DWORD",
        description: "Receives the current media position in milliseconds.",
    },
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap slot",
        description: "Slot whose DCMovieRenderer position is queried.",
    },
];
const GRAPH92_SET_BITMAP_MOVIE_VOLUME_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap slot",
        description: "Slot whose DCMovieRenderer volume is changed.",
    },
    NativeParameterSpec {
        name: "volume",
        kind: "i32 0..=128",
        description: "Target DirectShow volume scale validated by sub_44D450.",
    },
];

const GRAPH_PERSISTENT_TEXT_STYLE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "font_name",
        kind: "target string-registry selector",
        description: "Resolved through sub_468BB0 and copied to the 256-byte persistent face-name buffer; zero clears it.",
    },
    NativeParameterSpec {
        name: "ruby_height",
        kind: "i32",
        description: "Stored in target dword_565CE0; nonpositive selects the 91:98 font percentage.",
    },
    NativeParameterSpec {
        name: "ruby_font_field",
        kind: "i32",
        description: "Stored in target dword_565CE4; remaining consumers are not yet named.",
    },
    NativeParameterSpec {
        name: "ruby_x_offset",
        kind: "i32",
        description: "Stored in target dword_565CE8 and added by sub_437110.",
    },
    NativeParameterSpec {
        name: "ruby_y_offset",
        kind: "i32",
        description: "Stored in target dword_565CEC and applied to the body/ruby baseline.",
    },
    NativeParameterSpec {
        name: "text_override_0",
        kind: "i32",
        description: "Stored in target dword_507644; exact downstream meaning remains open.",
    },
    NativeParameterSpec {
        name: "text_override_1",
        kind: "i32",
        description: "Stored in target dword_507648; exact downstream meaning remains open.",
    },
];
const GRAPH90_REQUEST_REDRAW_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "full_redraw",
    kind: "boolean",
    description: "Nonzero requests a full redraw; full redraw dominates an already-pending partial request.",
}];
const GRAPH90_SET_SCHEDULER_GATE_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "gate",
    kind: "i32",
    description: "Raw target graph scheduler/update gate value.",
}];
const GRAPH90_SET_FRAME_RATE_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "frames_per_second",
    kind: "i32 1..=1000",
    description: "Accepted frame rate; target interval is max(1,1000/fps).",
}];
const GRAPH90_INITIALIZE_BITMAP_MEMORY_MANAGER_PARAMETERS: &[NativeParameterSpec] =
    &[NativeParameterSpec {
        name: "byte_limit",
        kind: "i32 0..=0x20000000",
        description: "Bitmap manager allocation limit.",
    }];
const GRAPH90_CREATE_WORK_BITMAP_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "bitmap",
    kind: "bitmap handle <0x4000",
    description: "Target bitmap slot created at current display dimensions and format.",
}];
const GRAPH90_CREATE_PRIORITIZED_WORK_BITMAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap handle <0x4000",
        description: "Target bitmap slot.",
    },
    NativeParameterSpec {
        name: "priority",
        kind: "u16 render priority",
        description: "Priority stored by the target in fixed-point form.",
    },
];
const GRAPH90_SET_CENTER_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "x",
        kind: "i32 coordinate",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32 coordinate",
        description: "Target-recovered BP argument.",
    },
];
const GRAPH90_SET_SYNC_LOAD_HOLD_PARAMETERS: &[NativeParameterSpec] =
    &[NativeParameterSpec {
        name: "hold_ms",
        kind: "i32 milliseconds",
        description: "dword_565AE0: how long Graph90:10 keeps loading synchronously after the first load (sub_402080); 0 disables the window.",
    }];
const GRAPH90_SET_DISPLAY_ENABLED_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "enabled",
    kind: "i32 gate",
    description: "Target graph/display enable value.",
}];
const GRAPH90_SET_DEFAULT_PRIORITY_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "priority",
    kind: "u16 render priority",
    description: "Accepted only below 0x10000.",
}];
const GRAPH90_SET_OBJECT_UPDATE_REDRAW_POLICY_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "enabled",
        kind: "boolean",
        description: "Whether completed display-object updates request redraw.",
    },
    NativeParameterSpec {
        name: "full_redraw",
        kind: "boolean",
        description: "Whether those requests are full redraws.",
    },
];
const GRAPH90_SET_CURRENT_BITMAP_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "bitmap",
    kind: "bitmap handle",
    description: "Bitmap manager current/primary bitmap.",
}];
const GRAPH90_SET_DISPLAY_OPTIONS_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "setting",
        kind: "i32",
        description: "First persistent display setting.",
    },
    NativeParameterSpec {
        name: "mode",
        kind: "i32 <=256",
        description: "Second setting; target validates the popped mode and invalidates display objects.",
    },
];
const GRAPH90_SET_RASTER_FORMAT_MODE_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "mode",
    kind: "i32 0..=3",
    description: "Selects one of four native raster layouts.",
}];
const GRAPH90_REGISTER_FONT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "font_name_id",
        kind: "string/name id",
        description: "Target font face identifier.",
    },
    NativeParameterSpec {
        name: "height",
        kind: "i32 4..=200",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "weight_percent",
        kind: "i32 25..=200",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "italic",
        kind: "boolean",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "font_id",
        kind: "i32 >=2",
        description: "Target user-font identifier.",
    },
];
const GRAPH90_SET_BITMAP_UNBLEND_COLOR_PARAMETERS: &[NativeParameterSpec] =
    &[NativeParameterSpec {
        name: "rgb",
        kind: "packed RGB color",
        description: "Background/unblend color stored in dword_565B18.",
    }];
const GRAPH90_LOAD_BITMAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap handle",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "archive",
        kind: "Shift-JIS archive name",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "resource",
        kind: "Shift-JIS resource name",
        description: "Target-recovered BP argument.",
    },
];
const GRAPH90_CREATE_BITMAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap handle",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "width",
        kind: "i32 pixels",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "height",
        kind: "i32 pixels",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "format",
        kind: "native bitmap format",
        description: "Target-recovered BP argument.",
    },
];
const GRAPH90_RELEASE_BITMAP_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "bitmap",
    kind: "bitmap handle",
    description: "Target-recovered BP argument.",
}];
const GRAPH90_FILL_BITMAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap handle",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "color",
        kind: "packed color",
        description: "Target-recovered BP argument.",
    },
];
const GRAPH90_CREATE_BITMAP_FROM_PIXELS_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap handle",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "width",
        kind: "i32 pixels",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "height",
        kind: "i32 pixels",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "format",
        kind: "native bitmap format",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "pixels",
        kind: "BP pointer",
        description: "Source packed pixel bytes.",
    },
];
const GRAPH90_COPY_BITMAP_PIXELS_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "BP pointer",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "written",
        kind: "BP i32 pointer",
        description: "Receives copied byte count.",
    },
    NativeParameterSpec {
        name: "capacity",
        kind: "i32 bytes",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap handle",
        description: "Target-recovered BP argument.",
    },
];
const GRAPH90_QUERY_BITMAP_INFO_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "info",
        kind: "BP pointer to 6 DWORD record",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap handle",
        description: "Target-recovered BP argument.",
    },
];
const GRAPH90_VALIDATE_BITMAP_FORMAT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap handle",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "expected_format",
        kind: "native bitmap format",
        description: "Target-recovered BP argument.",
    },
];
const GRAPH90_BLIT_BITMAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "arg0",
        kind: "i32 / bitmap operand",
        description: "Source-order argument 0; handler/helper role is preserved but finer pixel-formula semantics remain open.",
    },
    NativeParameterSpec {
        name: "arg1",
        kind: "i32 / bitmap operand",
        description: "Source-order argument 1; handler/helper role is preserved but finer pixel-formula semantics remain open.",
    },
    NativeParameterSpec {
        name: "arg2",
        kind: "i32 / bitmap operand",
        description: "Source-order argument 2; handler/helper role is preserved but finer pixel-formula semantics remain open.",
    },
    NativeParameterSpec {
        name: "arg3",
        kind: "i32 / bitmap operand",
        description: "Source-order argument 3; handler/helper role is preserved but finer pixel-formula semantics remain open.",
    },
    NativeParameterSpec {
        name: "arg4",
        kind: "i32 / bitmap operand",
        description: "Source-order argument 4; handler/helper role is preserved but finer pixel-formula semantics remain open.",
    },
    NativeParameterSpec {
        name: "arg5",
        kind: "i32 / bitmap operand",
        description: "Source-order argument 5; handler/helper role is preserved but finer pixel-formula semantics remain open.",
    },
];
const GRAPH90_SYNTHESIZE_BITMAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "destination", kind: "bitmap handle", description: "Required native bitmap slot." },
    NativeParameterSpec { name: "x", kind: "i32", description: "Native positional integer." },
    NativeParameterSpec { name: "y", kind: "i32", description: "Native positional integer." },
    NativeParameterSpec { name: "source", kind: "bitmap handle", description: "Required native bitmap slot." },
    NativeParameterSpec { name: "mask", kind: "gray bitmap or -1", description: "Source-sized format3 plane, or crossfade fallback." },
    NativeParameterSpec { name: "mask_shift", kind: "i32", description: "Native positional integer." },
    NativeParameterSpec { name: "transition", kind: "u32 <=256", description: "Crossfade/mask strength." },
];
const GRAPH90_COMPOSITE_BITMAPS_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "destination", kind: "bitmap handle", description: "Required native bitmap slot." },
    NativeParameterSpec { name: "source", kind: "bitmap handle", description: "Required native bitmap slot." },
    NativeParameterSpec { name: "map1", kind: "format4 bitmap", description: "Displacement map covering destination." },
    NativeParameterSpec { name: "map2", kind: "format4 bitmap or -1", description: "Optional equal-sized second vector map." },
    NativeParameterSpec { name: "alpha", kind: "u32 <=256", description: "Map interpolation strength." },
    NativeParameterSpec { name: "bilinear", kind: "BOOL", description: "Native sampling selector." },
];
const GRAPH90_COPY_BITMAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "destination", kind: "bitmap handle", description: "Required native bitmap slot." },
    NativeParameterSpec { name: "source", kind: "bitmap handle", description: "Required native bitmap slot." },
    NativeParameterSpec { name: "edge_mode", kind: "enum 0..1", description: "Black padding or edge clamp." },
    NativeParameterSpec { name: "radius", kind: "u32 <=256", description: "Horizontal blur radius, 256 clipped to255." },
];
const GRAPH90_SCALE_BITMAP_REGION_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "destination", kind: "bitmap handle", description: "Required native bitmap slot." },
    NativeParameterSpec { name: "dest_x", kind: "i32", description: "Native positional integer." },
    NativeParameterSpec { name: "dest_y", kind: "i32", description: "Native positional integer." },
    NativeParameterSpec { name: "dest_width", kind: "i32", description: "Native positional integer." },
    NativeParameterSpec { name: "dest_height", kind: "i32", description: "Native positional integer." },
    NativeParameterSpec { name: "source", kind: "bitmap handle", description: "Required native bitmap slot." },
    NativeParameterSpec { name: "source_x", kind: "i32", description: "Native positional integer." },
    NativeParameterSpec { name: "source_y", kind: "i32", description: "Native positional integer." },
    NativeParameterSpec { name: "source_width", kind: "i32", description: "Native positional integer." },
    NativeParameterSpec { name: "source_height", kind: "i32", description: "Native positional integer." },
];
const GRAPH90_TRANSFORM_BITMAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "destination", kind: "bitmap handle", description: "Required native bitmap slot." },
    NativeParameterSpec { name: "source", kind: "bitmap handle", description: "Required native bitmap slot." },
    NativeParameterSpec { name: "scale", kind: "nonzero 16.16", description: "Native inverse rotation scale." },
    NativeParameterSpec { name: "angle", kind: "16.16 degrees", description: "Rotation about bitmap center." },
];
const GRAPH90_BLIT_BITMAP_REGION_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "arg0",
        kind: "i32 / bitmap operand",
        description: "Source-order argument 0; handler/helper role is preserved but finer pixel-formula semantics remain open.",
    },
    NativeParameterSpec {
        name: "arg1",
        kind: "i32 / bitmap operand",
        description: "Source-order argument 1; handler/helper role is preserved but finer pixel-formula semantics remain open.",
    },
    NativeParameterSpec {
        name: "arg2",
        kind: "i32 / bitmap operand",
        description: "Source-order argument 2; handler/helper role is preserved but finer pixel-formula semantics remain open.",
    },
    NativeParameterSpec {
        name: "arg3",
        kind: "i32 / bitmap operand",
        description: "Source-order argument 3; handler/helper role is preserved but finer pixel-formula semantics remain open.",
    },
    NativeParameterSpec {
        name: "arg4",
        kind: "i32 / bitmap operand",
        description: "Source-order argument 4; handler/helper role is preserved but finer pixel-formula semantics remain open.",
    },
    NativeParameterSpec {
        name: "arg5",
        kind: "i32 / bitmap operand",
        description: "Source-order argument 5; handler/helper role is preserved but finer pixel-formula semantics remain open.",
    },
    NativeParameterSpec {
        name: "arg6",
        kind: "i32 / bitmap operand",
        description: "Source-order argument 6; handler/helper role is preserved but finer pixel-formula semantics remain open.",
    },
    NativeParameterSpec {
        name: "arg7",
        kind: "i32 / bitmap operand",
        description: "Source-order argument 7; handler/helper role is preserved but finer pixel-formula semantics remain open.",
    },
];
const GRAPH90_CREATE_BITMAP_REGION_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "bitmap handle",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "source",
        kind: "bitmap handle",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "i32 pixels",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32 pixels",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "width",
        kind: "i32 pixels",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "height",
        kind: "i32 pixels",
        description: "Target-recovered BP argument.",
    },
];
const GRAPH90_START_OBJECT_CONTROL_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "input_descriptor",
        kind: "u16-compatible input descriptor (0..=65535)",
        description: "CProcCtrlDspObj input-interrupt descriptor. sub_47A6A0 validates the public value through sub_497BB0 before sub_431E80 stores it.",
    },
    NativeParameterSpec {
        name: "input_enabled",
        kind: "boolean/i32",
        description: "Nonzero enables the CProcCtrlDspObj input-interrupt path installed by sub_431E80; zero disables input interruption.",
    },
    NativeParameterSpec {
        name: "frame_rate",
        kind: "nonzero i32 sampling denominator",
        description: "Sampling base passed as sub_491B40 argument a8. Zero is rejected with -2147483647 and the target formats the invalid-frame-rate error using this value.",
    },
    NativeParameterSpec {
        name: "duration_ms",
        kind: "i32 milliseconds",
        description: "Control duration passed to sub_431D10. Target sub_431D90 normalizes exactly zero to one millisecond; no stronger signed range is claimed here.",
    },
    NativeParameterSpec {
        name: "target_transparency",
        kind: "u32 0..=256",
        description: "Terminal CDspObj transparency. sub_47A6A0 validates it through sub_497DB0 before constructing CProcCtrlDspObj.",
    },
    NativeParameterSpec {
        name: "object",
        kind: "CDspObj/display-object handle",
        description: "Target display object supplied in ECX to sub_491B40. It must resolve through sub_443270; invalid objects make sub_491B40 return -1.",
    },
];
const GRAPH90_START_OBJECT_CONTROL_EX_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "object",
        kind: "display object handle",
        description: "CProcCtrlDspObj target object captured by sub_431D10.",
    },
    NativeParameterSpec {
        name: "target_alpha",
        kind: "0..=256",
        description: "Terminal transparency parameter.",
    },
    NativeParameterSpec {
        name: "duration_ms",
        kind: "milliseconds",
        description: "Base CProcCtrlDspObj duration; zero is normalized to one millisecond.",
    },
    NativeParameterSpec {
        name: "update_denominator",
        kind: "i32",
        description: "Sampling interval denominator used by sub_431D90.",
    },
    NativeParameterSpec {
        name: "update_numerator",
        kind: "i32",
        description: "Sampling interval numerator/enable used by sub_431D90.",
    },
    NativeParameterSpec {
        name: "input_enabled",
        kind: "boolean",
        description: "Enables CProcCtrlDspObj input-interrupt handling.",
    },
    NativeParameterSpec {
        name: "input_descriptor",
        kind: "input descriptor",
        description: "Packed target input scope used by CProcCtrlDspObj::Tick.",
    },
];
const GRAPH90_START_NODE_CONTROL_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "object",
        kind: "display object handle",
        description: "CProcCtrlDspObj target object captured by sub_431D50.",
    },
    NativeParameterSpec {
        name: "target_x",
        kind: "i32 pixels",
        description: "Terminal object X position.",
    },
    NativeParameterSpec {
        name: "target_y",
        kind: "i32 pixels",
        description: "Terminal object Y position.",
    },
    NativeParameterSpec {
        name: "position_curve",
        kind: "curve selector",
        description: "Position interpolation selector consumed by the CProcCtrlDspObj updater.",
    },
    NativeParameterSpec {
        name: "target_alpha",
        kind: "0..=256",
        description: "Terminal transparency parameter.",
    },
    NativeParameterSpec {
        name: "duration_ms",
        kind: "milliseconds",
        description: "Base CProcCtrlDspObj duration; zero is normalized to one millisecond.",
    },
    NativeParameterSpec {
        name: "update_denominator",
        kind: "i32",
        description: "Sampling denominator; this selector passes update_numerator=0.",
    },
    NativeParameterSpec {
        name: "input_enabled",
        kind: "boolean",
        description: "Enables CProcCtrlDspObj input-interrupt handling.",
    },
    NativeParameterSpec {
        name: "input_descriptor",
        kind: "input descriptor",
        description: "Packed target input scope used by CProcCtrlDspObj::Tick.",
    },
];
const GRAPH90_START_NODE_CONTROL_EX_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "arg0",
        kind: "control argument",
        description: "Source-order control argument 0; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
    NativeParameterSpec {
        name: "arg1",
        kind: "control argument",
        description: "Source-order control argument 1; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
    NativeParameterSpec {
        name: "arg2",
        kind: "control argument",
        description: "Source-order control argument 2; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
    NativeParameterSpec {
        name: "arg3",
        kind: "control argument",
        description: "Source-order control argument 3; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
    NativeParameterSpec {
        name: "arg4",
        kind: "control argument",
        description: "Source-order control argument 4; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
    NativeParameterSpec {
        name: "arg5",
        kind: "control argument",
        description: "Source-order control argument 5; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
    NativeParameterSpec {
        name: "arg6",
        kind: "control argument",
        description: "Source-order control argument 6; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
    NativeParameterSpec {
        name: "arg7",
        kind: "control argument",
        description: "Source-order control argument 7; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
    NativeParameterSpec {
        name: "arg8",
        kind: "control argument",
        description: "Source-order control argument 8; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
    NativeParameterSpec {
        name: "arg9",
        kind: "control argument",
        description: "Source-order control argument 9; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
];
const GRAPH90_START_SPECIAL_OBJECT_CONTROL_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "arg0",
        kind: "control argument",
        description: "Source-order control argument 0; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
    NativeParameterSpec {
        name: "arg1",
        kind: "control argument",
        description: "Source-order control argument 1; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
    NativeParameterSpec {
        name: "arg2",
        kind: "control argument",
        description: "Source-order control argument 2; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
    NativeParameterSpec {
        name: "arg3",
        kind: "control argument",
        description: "Source-order control argument 3; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
    NativeParameterSpec {
        name: "arg4",
        kind: "control argument",
        description: "Source-order control argument 4; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
    NativeParameterSpec {
        name: "arg5",
        kind: "control argument",
        description: "Source-order control argument 5; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
    NativeParameterSpec {
        name: "arg6",
        kind: "control argument",
        description: "Source-order control argument 6; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
    NativeParameterSpec {
        name: "arg7",
        kind: "control argument",
        description: "Source-order control argument 7; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
    NativeParameterSpec {
        name: "arg8",
        kind: "control argument",
        description: "Source-order control argument 8; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
    NativeParameterSpec {
        name: "arg9",
        kind: "control argument",
        description: "Source-order control argument 9; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
    NativeParameterSpec {
        name: "arg10",
        kind: "control argument",
        description: "Source-order control argument 10; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
    NativeParameterSpec {
        name: "arg11",
        kind: "control argument",
        description: "Source-order control argument 11; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
];
const GRAPH90_START_OBJECT_MOTION_CONTROL_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "arg0",
        kind: "control argument",
        description: "Source-order control argument 0; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
    NativeParameterSpec {
        name: "arg1",
        kind: "control argument",
        description: "Source-order control argument 1; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
    NativeParameterSpec {
        name: "arg2",
        kind: "control argument",
        description: "Source-order control argument 2; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
    NativeParameterSpec {
        name: "arg3",
        kind: "control argument",
        description: "Source-order control argument 3; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
    NativeParameterSpec {
        name: "arg4",
        kind: "control argument",
        description: "Source-order control argument 4; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
    NativeParameterSpec {
        name: "arg5",
        kind: "control argument",
        description: "Source-order control argument 5; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
    NativeParameterSpec {
        name: "arg6",
        kind: "control argument",
        description: "Source-order control argument 6; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
    NativeParameterSpec {
        name: "arg7",
        kind: "control argument",
        description: "Source-order control argument 7; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
    NativeParameterSpec {
        name: "arg8",
        kind: "control argument",
        description: "Source-order control argument 8; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
    NativeParameterSpec {
        name: "arg9",
        kind: "control argument",
        description: "Source-order control argument 9; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
    NativeParameterSpec {
        name: "arg10",
        kind: "control argument",
        description: "Source-order control argument 10; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
    NativeParameterSpec {
        name: "arg11",
        kind: "control argument",
        description: "Source-order control argument 11; exact field name remains unrecovered while constructor placement and procedure class are confirmed.",
    },
];
const GRAPH90_START_SPLINE_OBJECT_CONTROL_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "object",
        kind: "display object handle",
        description: "CProcCtrlDspObjBC target CDspObj.",
    },
    NativeParameterSpec {
        name: "point_count",
        kind: "i32",
        description: "Number of 16-byte spline point records; sub_432490 rejects non-positive counts.",
    },
    NativeParameterSpec {
        name: "points",
        kind: "pointer",
        description: "Pointer to point records; X/Y/Z feed three natural CSpline axes and the fourth DWORD is retained as endpoint metadata.",
    },
    NativeParameterSpec {
        name: "position_curve",
        kind: "curve id",
        description: "Position progress curve evaluated by sub_41A690 before CSpline sampling.",
    },
    NativeParameterSpec {
        name: "target_alpha",
        kind: "0..256 transparency",
        description: "Terminal CDspObj transparency.",
    },
    NativeParameterSpec {
        name: "packed_alpha_curve_scale",
        kind: "u32",
        description: "Low 16 bits are alpha curve; high 16 bits are alpha time scale, with zero meaning 0x10000.",
    },
    NativeParameterSpec {
        name: "fixed_parameter_target",
        kind: "i32",
        description: "Terminal selector -1 fixed parameter; negative preserves the captured start value.",
    },
    NativeParameterSpec {
        name: "duration_ms",
        kind: "milliseconds",
        description: "Base CProcCtrlDspObj duration; zero is normalized to one millisecond.",
    },
    NativeParameterSpec {
        name: "update_denominator",
        kind: "i32",
        description: "Denominator used by sub_431D90 for the native catch-up interval.",
    },
    NativeParameterSpec {
        name: "update_numerator",
        kind: "i32",
        description: "Numerator/enable value used by sub_431D90 for catch-up limiting.",
    },
    NativeParameterSpec {
        name: "input_enabled",
        kind: "boolean",
        description: "Enables CProcCtrlDspObj input-interrupt handling.",
    },
    NativeParameterSpec {
        name: "input_descriptor",
        kind: "input descriptor",
        description: "Packed target input scope used by CProcCtrlDspObj::Tick.",
    },
];
const GRAPH90_START_SHAKE_OBJECT_CONTROL_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "object",
        kind: "display object handle",
        description: "CProcShakeDspObj target CDspObj.",
    },
    NativeParameterSpec {
        name: "mode",
        kind: "0..5",
        description: "Shake axis/phase mode validated by sub_43C8A0.",
    },
    NativeParameterSpec {
        name: "amplitude",
        kind: "i32",
        description: "Initial shake amplitude.",
    },
    NativeParameterSpec {
        name: "frequency_hz",
        kind: "positive i32",
        description: "Wave frequency; sample_rate_hz must be at least this value.",
    },
    NativeParameterSpec {
        name: "cycles",
        kind: "positive i32",
        description: "Number of envelope cycles.",
    },
    NativeParameterSpec {
        name: "decay_percent",
        kind: "i32",
        description: "Per-cycle amplitude decay percentage.",
    },
    NativeParameterSpec {
        name: "sample_rate_hz",
        kind: "positive i32",
        description: "Native CProcedure update/sample rate.",
    },
    NativeParameterSpec {
        name: "input_enabled",
        kind: "boolean",
        description: "Enables CProcCtrlDspObj input-interrupt handling.",
    },
    NativeParameterSpec {
        name: "input_descriptor",
        kind: "input descriptor",
        description: "Packed target input scope used by CProcCtrlDspObj::Tick.",
    },
];
const GRAPH90_SET_OBJECT_DRAW_ENABLED_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "object",
        kind: "display object handle",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "enabled",
        kind: "boolean",
        description: "Target-recovered BP argument.",
    },
];
const GRAPH90_SET_OBJECT_ENABLED_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "object",
        kind: "display object handle",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "enabled",
        kind: "boolean",
        description: "Target-recovered BP argument.",
    },
];
const GRAPH90_SET_OBJECT_POSITION_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "object",
        kind: "display object handle",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "i32 coordinate",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32 coordinate",
        description: "Target-recovered BP argument.",
    },
];
const GRAPH90_SET_OBJECT_MASK_ALPHA_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "object",
        kind: "display object handle",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "mask_alpha",
        kind: "i32 transparency",
        description: "Target-recovered BP argument.",
    },
];
const GRAPH90_SET_OBJECT_FIXED_PARAMETER_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "object",
        kind: "display object handle",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "value",
        kind: "i32",
        description: "Target-recovered BP argument.",
    },
];
const GRAPH90_SET_OBJECT_SECONDARY_OFFSET_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "object",
        kind: "display object handle",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "i32 coordinate",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32 coordinate",
        description: "Target-recovered BP argument.",
    },
];
const GRAPH90_SET_OBJECT_PRIMARY_OFFSET_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "object",
        kind: "display object handle",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "i32 coordinate",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32 coordinate",
        description: "Target-recovered BP argument.",
    },
];
const GRAPH90_SET_OBJECT_PROPERTY_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "object",
        kind: "display object handle",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "property",
        kind: "i32 selector",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "value",
        kind: "i32",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "extra",
        kind: "i32",
        description: "Target-recovered BP argument.",
    },
];
const GRAPH90_SET_OBJECT_ALPHA_MULTIPLIER_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "object",
        kind: "display object handle",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "multiplier",
        kind: "i32 0..=256",
        description: "Target-recovered BP argument.",
    },
];
const GRAPH90_SET_OBJECT_PRIORITY_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "object",
        kind: "display object handle",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "priority",
        kind: "u16 render priority",
        description: "Target-recovered BP argument.",
    },
];
const GRAPH90_SET_OBJECT_HIT_MASK_BITMAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "object",
        kind: "display object handle",
        description: "Target-recovered BP argument.",
    },
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap handle or -1/-2",
        description: "-1 clears; -2 generates from the object bitmap; nonnegative selects a bitmap descriptor.",
    },
];
const GRAPH90_HIT_TEST_OBJECT_AT_POINTER_PARAMETERS: &[NativeParameterSpec] =
    &[NativeParameterSpec {
        name: "object",
        kind: "display object handle",
        description: "Target-recovered BP argument.",
    }];
const GRAPH90_INVOKE_UNSUPPORTED_OBJECT_EXTENSION_PARAMETERS: &[NativeParameterSpec] =
    &[NativeParameterSpec {
        name: "object",
        kind: "display object handle",
        description: "Target-recovered BP argument.",
    }];

const GRAPH_SET_OBJECT_ALPHA_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "object",
        kind: "graph/display object handle",
        description: "Target display object whose CDspObj transparency field is updated.",
    },
    NativeParameterSpec {
        name: "alpha_parameter",
        kind: "i32 transparency in 1/256 units",
        description: "Target setter stores this value at CDspObj+0xAC; 0 is opaque and 256 is transparent.",
    },
];
const GRAPH90_SET_CURRENT_OBJECT_BITMAP_PARAMETERS: &[NativeParameterSpec] =
    &[NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap handle",
        description: "Bitmap bound to the target current display object.",
    }];
const GRAPH90_CONFIGURE_CURRENT_OBJECT_DUAL_BITMAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "primary_bitmap",
        kind: "bitmap handle",
        description: "Primary bitmap resource.",
    },
    NativeParameterSpec {
        name: "secondary_bitmap",
        kind: "bitmap handle",
        description: "Secondary bitmap resource.",
    },
    NativeParameterSpec {
        name: "alpha_parameter",
        kind: "i32 transparency parameter",
        description: "Target CDspObj transparency parameter.",
    },
];
const GRAPH90_CONFIGURE_CURRENT_OBJECT_QUAD_BITMAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "bitmap_0",
        kind: "bitmap handle",
        description: "Bitmap resource 0.",
    },
    NativeParameterSpec {
        name: "bitmap_1",
        kind: "bitmap handle",
        description: "Bitmap resource 1.",
    },
    NativeParameterSpec {
        name: "bitmap_2",
        kind: "bitmap handle",
        description: "Bitmap resource 2.",
    },
    NativeParameterSpec {
        name: "bitmap_3",
        kind: "bitmap handle",
        description: "Bitmap resource 3.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "i32 coordinate",
        description: "Object X coordinate.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32 coordinate",
        description: "Object Y coordinate.",
    },
];
const GRAPH90_CONFIGURE_CURRENT_OBJECT_SPRITE_MASK_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "primary_x",
        kind: "i32 source coordinate",
        description: "Primary bitmap source X offset stored at CDspObjBackF+0x13c.",
    },
    NativeParameterSpec {
        name: "primary_y",
        kind: "i32 source coordinate",
        description: "Primary bitmap source Y offset stored at CDspObjBackF+0x140.",
    },
    NativeParameterSpec {
        name: "primary_resource",
        kind: "bitmap handle",
        description: "Required primary bitmap; the target stores its generation at +0x148.",
    },
    NativeParameterSpec {
        name: "secondary_x",
        kind: "i32 source coordinate",
        description: "Secondary bitmap source X offset stored at CDspObjBackF+0x14c.",
    },
    NativeParameterSpec {
        name: "secondary_y",
        kind: "i32 source coordinate",
        description: "Secondary bitmap source Y offset stored at CDspObjBackF+0x150.",
    },
    NativeParameterSpec {
        name: "secondary_resource",
        kind: "bitmap handle or sentinel",
        description: "Secondary bitmap, or target sentinel 0x7000/0x7001/0x7fff/-1.",
    },
    NativeParameterSpec {
        name: "mask_resource",
        kind: "format-3 bitmap handle or -1",
        description: "Optional format-3 mask bitmap stored at CDspObjBackF+0x15c.",
    },
    NativeParameterSpec {
        name: "mask_parameter",
        kind: "i32 mask selector",
        description: "Mask interpretation parameter stored at CDspObjBackF+0x160.",
    },
    NativeParameterSpec {
        name: "alpha_parameter",
        kind: "i32 transparency parameter",
        description: "Target CDspObj transparency DWORD; sub_41B620 stores it verbatim.",
    },
];
const GRAPH90_CONFIGURE_CURRENT_OBJECT_FRAME_TABLE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "frame_count",
        kind: "i32 2..=32",
        description: "Number of resource entries copied by the target.",
    },
    NativeParameterSpec {
        name: "frame_table",
        kind: "BP pointer to resource handles",
        description: "VM array containing frame/resource handles.",
    },
    NativeParameterSpec {
        name: "alpha_parameter",
        kind: "i32 transparency parameter",
        description: "Target CDspObj transparency parameter.",
    },
];
const GRAPH90_CONFIGURE_CURRENT_OBJECT_RESOURCE_TRIPLET_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "resource_0",
        kind: "resource handle",
        description: "First resource.",
    },
    NativeParameterSpec {
        name: "resource_1",
        kind: "resource handle",
        description: "Second resource.",
    },
    NativeParameterSpec {
        name: "resource_2",
        kind: "resource handle",
        description: "Third resource.",
    },
    NativeParameterSpec {
        name: "alpha_parameter",
        kind: "i32 transparency parameter",
        description: "Target CDspObj transparency parameter.",
    },
    NativeParameterSpec {
        name: "extra_parameter",
        kind: "i32 target field",
        description: "Stored in the target current-object extension field at +0x154.",
    },
];
const GRAPH90_CONFIGURE_CURRENT_OBJECT_BITMAP_MODE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap handle",
        description: "Target bitmap resource.",
    },
    NativeParameterSpec {
        name: "mode",
        kind: "i32 0..=1",
        description: "Target current-object bitmap mode.",
    },
    NativeParameterSpec {
        name: "alpha_parameter",
        kind: "i32 transparency parameter",
        description: "Target CDspObj transparency parameter.",
    },
];
const GRAPH90_CONFIGURE_CURRENT_OBJECT_VM_EFFECT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap handle",
        description: "Primary bitmap resource.",
    },
    NativeParameterSpec {
        name: "auxiliary_resource",
        kind: "resource handle",
        description: "Auxiliary target resource.",
    },
    NativeParameterSpec {
        name: "record_count",
        kind: "positive i32",
        description: "Number of four-sample wave records.",
    },
    NativeParameterSpec {
        name: "record_pointer",
        kind: "BP pointer",
        description: "Pointer to target effect records in VM memory.",
    },
    NativeParameterSpec {
        name: "alpha_parameter",
        kind: "i32 transparency parameter",
        description: "Target CDspObj transparency parameter.",
    },
];
const GRAPH90_CONFIGURE_CURRENT_OBJECT_BITMAP_SIZE_POSITION_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap handle",
        description: "Target bitmap resource.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "i32 coordinate",
        description: "Object X coordinate.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32 coordinate",
        description: "Object Y coordinate.",
    },
    NativeParameterSpec {
        name: "width",
        kind: "i32 >= 2",
        description: "Configured width.",
    },
    NativeParameterSpec {
        name: "height",
        kind: "i32 >= 2",
        description: "Configured height.",
    },
];
const GRAPH90_CONFIGURE_CURRENT_OBJECT_BITMAP_SIZE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap handle",
        description: "Target bitmap resource.",
    },
    NativeParameterSpec {
        name: "scale_16_16",
        kind: "nonzero i32",
        description: "Nonzero unsigned 16.16 scale of the rotating background.",
    },
    NativeParameterSpec {
        name: "rotation_16_16",
        kind: "i32 16.16 degrees",
        description: "Rotation around the bitmap center.",
    },
];
const GRAPH90_CONFIGURE_CURRENT_OBJECT_BLIT_SOURCES_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "primary_resource",
        kind: "resource handle",
        description: "Primary source resource.",
    },
    NativeParameterSpec {
        name: "secondary_resource",
        kind: "resource handle or target sentinel",
        description: "Secondary source resource.",
    },
    NativeParameterSpec {
        name: "mode",
        kind: "i32 required zero",
        description: "Target selector requires zero in this slot.",
    },
    NativeParameterSpec {
        name: "alpha_parameter",
        kind: "i32 transparency parameter",
        description: "Target CDspObj transparency parameter.",
    },
    NativeParameterSpec {
        name: "flag",
        kind: "i32 0..=1",
        description: "Target source-composition flag.",
    },
];
const GRAPH90_SET_CURRENT_OBJECT_RENDER_CONTROLS_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "draw_enabled",
        kind: "boolean-like i32",
        description: "Forwarded through CDspObj vtable slot +4.",
    },
    NativeParameterSpec {
        name: "render_control",
        kind: "i32 target control",
        description: "Forwarded through the current-object extension setter at vtable +120.",
    },
];
const GRAPH90_GET_CURRENT_OBJECT_MODE_PARAMETERS: &[NativeParameterSpec] = &[];
const GRAPH90_CREATE_SPRITE_OBJECT_PARAMETERS: &[NativeParameterSpec] = &[];
const GRAPH90_RELEASE_SPRITE_OBJECT_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "sprite",
    kind: "0x80000000-tagged sprite handle",
    description: "Sprite object to release.",
}];
const GRAPH90_REFRESH_SPRITE_OBJECT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "sprite",
        kind: "sprite handle",
        description: "Target CDspObjSprite.",
    },
    NativeParameterSpec {
        name: "reserved_1",
        kind: "i32 ignored by target core",
        description: "Consumed by the public ABI; not used by sub_43EEE0/sub_428C00.",
    },
    NativeParameterSpec {
        name: "reserved_2",
        kind: "i32 ignored by target core",
        description: "Consumed by the public ABI; not used by sub_43EEE0/sub_428C00.",
    },
    NativeParameterSpec {
        name: "reserved_3",
        kind: "i32 ignored by target core",
        description: "Consumed by the public ABI; not used by sub_43EEE0/sub_428C00.",
    },
    NativeParameterSpec {
        name: "reserved_4",
        kind: "i32 ignored by target core",
        description: "Consumed by the public ABI; not used by sub_43EEE0/sub_428C00.",
    },
];
const GRAPH90_SET_SPRITE_ENABLED_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "sprite",
        kind: "sprite handle",
        description: "Target CDspObjSprite.",
    },
    NativeParameterSpec {
        name: "enabled",
        kind: "boolean-like i32",
        description: "Inherited enabled gate.",
    },
];
const GRAPH90_SET_SPRITE_AUX_BITMAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "sprite",
        kind: "sprite handle",
        description: "Target CDspObjSprite.",
    },
    NativeParameterSpec {
        name: "bitmap",
        kind: "format-2/3 bitmap handle or -1",
        description: "Auxiliary bitmap; -1 clears it.",
    },
];
const GRAPH90_CONFIGURE_SPRITE_SINGLE_BITMAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "sprite",
        kind: "sprite handle",
        description: "Target CDspObjSprite.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "i32 coordinate",
        description: "Sprite X.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32 coordinate",
        description: "Sprite Y.",
    },
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap handle",
        description: "Primary bitmap.",
    },
    NativeParameterSpec {
        name: "blend_mode",
        kind: "i32 blend selector",
        description: "Target blend mode.",
    },
    NativeParameterSpec {
        name: "alpha_parameter",
        kind: "i32 transparency parameter",
        description: "Target transparency parameter.",
    },
    NativeParameterSpec {
        name: "priority",
        kind: "u16 render priority",
        description: "Display-chain priority.",
    },
];
const GRAPH90_REPLACE_SPRITE_BITMAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "sprite",
        kind: "sprite handle",
        description: "Existing CDspObjSprite whose primary bitmap is replaced.",
    },
    NativeParameterSpec {
        name: "primary_bitmap",
        kind: "bitmap handle",
        description: "New primary bitmap; supported current modes rebuild their geometry around this bitmap.",
    },
];
const GRAPH90_CONFIGURE_SPRITE_DUAL_BITMAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "sprite",
        kind: "sprite handle",
        description: "Existing CDspObjSprite created by 0x90:0x50.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "i32 coordinate",
        description: "Sprite X.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32 coordinate",
        description: "Sprite Y.",
    },
    NativeParameterSpec {
        name: "primary_bitmap",
        kind: "bitmap handle",
        description: "First image.",
    },
    NativeParameterSpec {
        name: "secondary_bitmap",
        kind: "bitmap handle",
        description: "Second image.",
    },
    NativeParameterSpec {
        name: "transition_value",
        kind: "i32 target transition field",
        description: "Mode-1 transition parameter.",
    },
    NativeParameterSpec {
        name: "alpha_parameter",
        kind: "i32 transparency parameter",
        description: "Target transparency parameter.",
    },
    NativeParameterSpec {
        name: "priority",
        kind: "u16 render priority",
        description: "Display-chain priority.",
    },
    NativeParameterSpec {
        name: "transition_mode",
        kind: "i32 target selector",
        description: "Mode-1 transition selector.",
    },
];
const GRAPH90_CONFIGURE_SPRITE_SCALED_BITMAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "sprite",
        kind: "sprite handle",
        description: "Target CDspObjSprite.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "i32 pixel coordinate",
        description: "Ordinary CDspObj X anchor written through vtable+0x2C before mode-2 raster-origin correction.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32 pixel coordinate",
        description: "Ordinary CDspObj Y anchor written through vtable+0x2C before mode-2 raster-origin correction.",
    },
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap handle",
        description: "Primary bitmap; target rejects an invalid bitmap descriptor.",
    },
    NativeParameterSpec {
        name: "transform_x",
        kind: "i32 pixel value",
        description: "Mode-2 transform X term; sub_4275B0 converts it to signed 16.16 and stores it at CDspObjSprite+0x248. It is not a source-rectangle X.",
    },
    NativeParameterSpec {
        name: "transform_y",
        kind: "i32 pixel value",
        description: "Mode-2 transform Y term; sub_4275B0 converts it to signed 16.16 and stores it at CDspObjSprite+0x24C. It is not a source-rectangle Y.",
    },
    NativeParameterSpec {
        name: "rotation_16_16",
        kind: "signed 16.16 degrees",
        description: "Mode-2 rotation term stored at CDspObjSprite+0x250 and consumed by the transformed-bounds helper.",
    },
    NativeParameterSpec {
        name: "scale_x_16_16",
        kind: "signed 16.16 ratio",
        description: "Horizontal stretch ratio. The public wrapper reports an invalid-stretch error for rejected X/Y ratios.",
    },
    NativeParameterSpec {
        name: "scale_y_16_16",
        kind: "signed 16.16 ratio",
        description: "Vertical stretch ratio.",
    },
    NativeParameterSpec {
        name: "raster_mode_flag",
        kind: "i32 flag",
        description: "Mode-2 raster/blit selector stored at CDspObjSprite+0x280 and forwarded to the affine pixel path; exact higher-level label remains unclosed.",
    },
    NativeParameterSpec {
        name: "blend_mode",
        kind: "i32 blend selector",
        description: "Target blend mode.",
    },
    NativeParameterSpec {
        name: "alpha_parameter",
        kind: "i32 transparency parameter",
        description: "Target transparency parameter; 0 is opaque and 256 fully transparent.",
    },
    NativeParameterSpec {
        name: "priority",
        kind: "u16 render priority",
        description: "Display-chain priority.",
    },
];
const GRAPH90_CONFIGURE_SPRITE_MASKED_BITMAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "sprite",
        kind: "sprite handle",
        description: "Target CDspObjSprite.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "i32 coordinate",
        description: "Sprite X.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32 coordinate",
        description: "Sprite Y.",
    },
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap handle",
        description: "Primary bitmap.",
    },
    NativeParameterSpec {
        name: "mask_resource",
        kind: "format-3 resource handle",
        description: "Mask/auxiliary resource.",
    },
    NativeParameterSpec {
        name: "mask_parameter",
        kind: "i32 target field",
        description: "Mode-3 mask parameter.",
    },
    NativeParameterSpec {
        name: "fixed_parameter",
        kind: "i32 converted to 16.16",
        description: "Mode-3 fixed-point parameter.",
    },
    NativeParameterSpec {
        name: "blend_mode",
        kind: "i32 blend selector",
        description: "Target blend mode.",
    },
    NativeParameterSpec {
        name: "alpha_parameter",
        kind: "i32 transparency parameter",
        description: "Target transparency parameter.",
    },
    NativeParameterSpec {
        name: "priority",
        kind: "u16 render priority",
        description: "Display-chain priority.",
    },
];
const GRAPH90_CONFIGURE_SPRITE_VM_EFFECT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "sprite",
        kind: "sprite handle",
        description: "Target CDspObjSprite.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "i32 coordinate",
        description: "Sprite X.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32 coordinate",
        description: "Sprite Y.",
    },
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap handle",
        description: "Primary bitmap.",
    },
    NativeParameterSpec {
        name: "effect_resource",
        kind: "format-6 resource handle",
        description: "Target effect resource.",
    },
    NativeParameterSpec {
        name: "record_count",
        kind: "positive i32",
        description: "Number of four-sample wave records.",
    },
    NativeParameterSpec {
        name: "record_pointer",
        kind: "BP pointer",
        description: "Wave-table slot (0..=7), not a BP pointer.",
    },
    NativeParameterSpec {
        name: "alpha_parameter",
        kind: "i32 transparency parameter",
        description: "Target transparency parameter.",
    },
    NativeParameterSpec {
        name: "mask_alpha",
        kind: "i32 mask alpha",
        description: "Target mask-alpha field.",
    },
    NativeParameterSpec {
        name: "priority",
        kind: "u16 render priority",
        description: "Display-chain priority.",
    },
];
const GRAPH90_CONFIGURE_SPRITE_TRANSFORM_MODE5_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "sprite",
        kind: "sprite handle",
        description: "Target CDspObjSprite.",
    },
    NativeParameterSpec {
        name: "x_16_16",
        kind: "signed 16.16 coordinate",
        description: "CDspObj +0x4C X written by vtable+60 (sub_41B370); not an integer-pixel raster coordinate.",
    },
    NativeParameterSpec {
        name: "y_16_16",
        kind: "signed 16.16 coordinate",
        description: "CDspObj +0x50 Y written by vtable+60 (sub_41B370); target propagates this fixed-point vector through the display-object chain.",
    },
    NativeParameterSpec {
        name: "z_16_16",
        kind: "signed 16.16 coordinate",
        description: "CDspObj +0x54 Z written by vtable+60. Mode 5 feeds the accumulated value to sub_41AAA0 for perspective scaling; mode 6 uses it in the projected quad path.",
    },
    NativeParameterSpec {
        name: "primary_bitmap",
        kind: "bitmap handle",
        description: "Required primary bitmap resolved by sub_427AA0.",
    },
    NativeParameterSpec {
        name: "secondary_bitmap",
        kind: "bitmap handle or -1",
        description: "Optional same-format secondary bitmap.",
    },
    NativeParameterSpec {
        name: "transition_value",
        kind: "i32 0..=256",
        description: "Stored only when the secondary bitmap is present.",
    },
    NativeParameterSpec {
        name: "secondary_parameter",
        kind: "i32 target field",
        description: "Stored beside the optional secondary bitmap, or forced to -1 when absent.",
    },
    NativeParameterSpec {
        name: "bitmap_origin_x",
        kind: "i32 source coordinate converted to 16.16",
        description: "Source-space X origin/anchor stored at CDspObjSprite+0x284. sub_4258D0 passes it to sub_417730/sub_416750.",
    },
    NativeParameterSpec {
        name: "bitmap_origin_y",
        kind: "i32 source coordinate converted to 16.16",
        description: "Source-space Y origin/anchor stored at CDspObjSprite+0x288. This field is independent of bitmap-registry +0x28/+0x2C; any script use of the CBG reference point reaches it explicitly through the bitmap query/configuration path.",
    },
    NativeParameterSpec {
        name: "rotation_16_16",
        kind: "signed 16.16 degrees",
        description: "Rotation stored at CDspObjSprite+0x28C and consumed by sub_429220 and sub_416750.",
    },
    NativeParameterSpec {
        name: "perspective",
        kind: "positive i32 projection distance",
        description: "Projection distance stored at CDspObjSprite+0x278; sub_41AAA0 combines it with Z to derive the 16.16 scale.",
    },
    NativeParameterSpec {
        name: "project_position",
        kind: "bool i32",
        description: "CDspObjSprite+0x27C gate. Nonzero makes sub_429AF0 perspective-project X/Y by the Z-derived scale before resolving ordinary object position.",
    },
    NativeParameterSpec {
        name: "interpolation",
        kind: "bool i32",
        description: "CDspObjSprite+0x280 sampling selector passed by case-5 drawing to sub_417730: 0 selects nearest (sub_418280), nonzero selects bilinear (sub_417C50).",
    },
    NativeParameterSpec {
        name: "blend_mode",
        kind: "i32 blend selector",
        description: "Validated by sub_497C40.",
    },
    NativeParameterSpec {
        name: "alpha_parameter",
        kind: "i32 transparency parameter",
        description: "Validated in the target 0..=256 domain.",
    },
    NativeParameterSpec {
        name: "priority",
        kind: "u16 render priority",
        description: "Display-chain priority.",
    },
];
const GRAPH90_CONFIGURE_SPRITE_TRANSFORM_MODE6_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "sprite",
        kind: "sprite handle",
        description: "Target CDspObjSprite.",
    },
    NativeParameterSpec {
        name: "x_16_16",
        kind: "signed 16.16 coordinate",
        description: "CDspObj +0x4C X written by vtable+60 (sub_41B370); not an integer-pixel raster coordinate.",
    },
    NativeParameterSpec {
        name: "y_16_16",
        kind: "signed 16.16 coordinate",
        description: "CDspObj +0x50 Y written by vtable+60 (sub_41B370); target propagates this fixed-point vector through the display-object chain.",
    },
    NativeParameterSpec {
        name: "z_16_16",
        kind: "signed 16.16 coordinate",
        description: "CDspObj +0x54 Z written by vtable+60. Mode 5 feeds the accumulated value to sub_41AAA0 for perspective scaling; mode 6 uses it in the projected quad path.",
    },
    NativeParameterSpec {
        name: "primary_bitmap",
        kind: "bitmap handle",
        description: "Required primary bitmap resolved by sub_427D90.",
    },
    NativeParameterSpec {
        name: "secondary_bitmap",
        kind: "bitmap handle or -1",
        description: "Optional same-format secondary bitmap.",
    },
    NativeParameterSpec {
        name: "transition_value",
        kind: "i32 0..=256",
        description: "Stored only when the secondary bitmap is present.",
    },
    NativeParameterSpec {
        name: "secondary_parameter",
        kind: "i32 target field",
        description: "Stored beside the optional secondary bitmap, or forced to -1 when absent.",
    },
    NativeParameterSpec {
        name: "anchor_x", kind: "integer pixels", description: "Anchor converted to16.16.",
    },
    NativeParameterSpec {
        name: "anchor_y", kind: "integer pixels", description: "Anchor converted to16.16.",
    },
    NativeParameterSpec {
        name: "pitch", kind: "16.16 degrees", description: "X-axis Euler angle.",
    },
    NativeParameterSpec {
        name: "heading", kind: "16.16 degrees", description: "Y-axis Euler angle.",
    },
    NativeParameterSpec {
        name: "bank", kind: "16.16 degrees", description: "Z-axis Euler angle.",
    },
    NativeParameterSpec {
        name: "rotation_order", kind: "enum0..5", description: "Euler-axis permutation.",
    },
    NativeParameterSpec {
        name: "distance", kind: "i32", description: "Projection focal distance.",
    },
    NativeParameterSpec {
        name: "project_position", kind: "BOOL", description: "Translate before or after perspective.",
    },
    NativeParameterSpec {
        name: "stored_sampling_flag", kind: "i32", description: "Stored flag; case6 always uses bilinear sampling.",
    },
    NativeParameterSpec {
        name: "blend_mode",
        kind: "i32 blend selector",
        description: "Validated by sub_497C40.",
    },
    NativeParameterSpec {
        name: "alpha_parameter",
        kind: "i32 transparency parameter",
        description: "Validated in the target 0..=256 domain.",
    },
    NativeParameterSpec {
        name: "priority",
        kind: "u16 render priority",
        description: "Display-chain priority.",
    },
];
const GRAPH90_CREATE_FILTER_OBJECT_PARAMETERS: &[NativeParameterSpec] = &[];
const GRAPH90_RELEASE_FILTER_OBJECT_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "filter",
    kind: "0x90000000-tagged filter handle",
    description: "Filter object to release.",
}];
const GRAPH90_SET_FILTER_ENABLED_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "filter",
        kind: "filter handle",
        description: "Target CDspObjFilter.",
    },
    NativeParameterSpec {
        name: "enabled",
        kind: "boolean-like i32",
        description: "Inherited enabled gate.",
    },
];
const GRAPH90_CONFIGURE_FILTER_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "filter", kind: "filter handle", description: "CDspObjFilter." },
    NativeParameterSpec { name: "color", kind: "packed RGB", description: "Mode0 color blend." },
    NativeParameterSpec { name: "strength", kind: "u32 <=256", description: "Native alpha after mask/multiplier." },
    NativeParameterSpec { name: "priority", kind: "u32 <65536", description: "Display-chain priority." },
];
const GRAPH90_CONFIGURE_FILTER_WITH_MASK_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "filter", kind: "filter handle", description: "CDspObjFilter." },
    NativeParameterSpec { name: "mode", kind: "enum 0..3", description: "Color blend, brightness, luma colorization or XOR blend. Other values draw nothing." },
    NativeParameterSpec { name: "color", kind: "packed RGB", description: "Filter color." },
    NativeParameterSpec { name: "mask", kind: "format3 bitmap or -1", description: "Optional screen-sized gray mask; overrides mode." },
    NativeParameterSpec { name: "mask_shift", kind: "i32", description: "Native shift count uses low five bits." },
    NativeParameterSpec { name: "strength", kind: "u32 <=256", description: "Native filter strength." },
    NativeParameterSpec { name: "priority", kind: "u32 <65536", description: "Display-chain priority." },
];
const GRAPH90_CREATE_MAP_OBJECT_PARAMETERS: &[NativeParameterSpec] = &[];
const GRAPH90_RELEASE_MAP_OBJECT_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "map",
    kind: "0xA0000000-tagged map handle",
    description: "Map object to release.",
}];
const GRAPH90_SET_MAP_ENABLED_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "map",
        kind: "map handle",
        description: "Target CDspObjMap.",
    },
    NativeParameterSpec {
        name: "enabled",
        kind: "boolean-like i32",
        description: "Inherited enabled gate.",
    },
];
const GRAPH90_CONFIGURE_MAP_OBJECT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "map",
        kind: "map handle",
        description: "Target CDspObjMap.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "i32 coordinate",
        description: "Map X.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32 coordinate",
        description: "Map Y.",
    },
    NativeParameterSpec {
        name: "resource",
        kind: "bitmap/resource handle",
        description: "Map tile resource.",
    },
    NativeParameterSpec {
        name: "blend_mode",
        kind: "i32 blend selector",
        description: "Target blend mode.",
    },
    NativeParameterSpec {
        name: "alpha_parameter",
        kind: "i32 transparency parameter",
        description: "Target transparency parameter.",
    },
    NativeParameterSpec {
        name: "priority",
        kind: "u16 render priority",
        description: "Display-chain priority.",
    },
];
const GRAPH90_INITIALIZE_MAP_GRID_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "map", kind: "map handle", description: "CDspObjMap." },
    NativeParameterSpec { name: "columns", kind: "positive u32", description: "At most1024/cell_width." },
    NativeParameterSpec { name: "rows", kind: "positive u32", description: "At most768/cell_height." },
    NativeParameterSpec { name: "cell_width", kind: "u32 1..256", description: "Tile width." },
    NativeParameterSpec { name: "cell_height", kind: "u32 1..256", description: "Tile height." },
];
const GRAPH90_UPLOAD_MAP_TILE_DATA_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "map",
        kind: "map handle",
        description: "Target CDspObjMap.",
    },
    NativeParameterSpec {
        name: "width",
        kind: "i32 extent",
        description: "Uploaded map width.",
    },
    NativeParameterSpec {
        name: "height",
        kind: "i32 extent",
        description: "Uploaded map height.",
    },
    NativeParameterSpec {
        name: "tile_data",
        kind: "BP pointer to u16 entries",
        description: "VM tile-index data.",
    },
];
const GRAPH90_SET_MAP_VIEWPORT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "map",
        kind: "map handle",
        description: "Target CDspObjMap.",
    },
    NativeParameterSpec {
        name: "source_x",
        kind: "i32 coordinate",
        description: "Source X.",
    },
    NativeParameterSpec {
        name: "source_y",
        kind: "i32 coordinate",
        description: "Source Y.",
    },
    NativeParameterSpec {
        name: "cell_offset_x",
        kind: "i32 offset",
        description: "Cell offset X.",
    },
    NativeParameterSpec {
        name: "cell_offset_y",
        kind: "i32 offset",
        description: "Cell offset Y.",
    },
    NativeParameterSpec {
        name: "wrap",
        kind: "boolean-like i32",
        description: "Target wrapping selector.",
    },
];
const GRAPH90_REPLACE_MAP_TILE_ID_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "map", kind: "map handle", description: "CDspObjMap." },
    NativeParameterSpec { name: "tile_id", kind: "i32", description: "Invalidate matching current IDs using complemented desired IDs; extra desired-cursor advance on matches is preserved." },
];
const GRAPH90_CREATE_WINDOW_OBJECT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "width",
        kind: "i32 native width unit",
        description: "Values below 32 are multiplied by 32; resulting width must be 1..=1920.",
    },
    NativeParameterSpec {
        name: "height",
        kind: "i32 native height unit",
        description: "Values below 32 are multiplied by 32; resulting height must be 1..=32768.",
    },
];
const GRAPH90_WINDOW_HANDLE_PARAMETER: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "window",
    kind: "0xB0000000-tagged window handle",
    description: "Target CDspObjWindow handle with slot index below 16.",
}];
const GRAPH90_SET_WINDOW_COMPOSITION_ORDER_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "window",
        kind: "window handle",
        description: "Target CDspObjWindow.",
    },
    NativeParameterSpec {
        name: "mode",
        kind: "i32 enum 0..=5",
        description: "Selects one of six permutations of the three composition passes.",
    },
];
const GRAPH90_SET_WINDOW_ENABLED_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "window",
        kind: "window handle",
        description: "Target CDspObjWindow.",
    },
    NativeParameterSpec {
        name: "enabled",
        kind: "bool/i32",
        description: "Window draw-enable gate.",
    },
];
const GRAPH90_CONFIGURE_WINDOW_OBJECT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "window",
        kind: "window handle",
        description: "Target CDspObjWindow.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "i32 coordinate",
        description: "Window X position.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32 coordinate",
        description: "Window Y position.",
    },
    NativeParameterSpec {
        name: "blend_mode",
        kind: "target blend selector",
        description: "Validated by the target blend-mode helper.",
    },
    NativeParameterSpec {
        name: "transparency",
        kind: "i32 0..=256",
        description: "Native transparency parameter forwarded to the alpha virtual setter.",
    },
    NativeParameterSpec {
        name: "reserved_validated",
        kind: "i32 0..=256",
        description: "Validated by the public handler but not passed to the target core.",
    },
    NativeParameterSpec {
        name: "priority",
        kind: "u16 render priority",
        description: "Window display-chain priority.",
    },
];
const GRAPH90_SET_WINDOW_ISOLATED_COMPOSITION_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "window",
        kind: "window handle",
        description: "Target CDspObjWindow.",
    },
    NativeParameterSpec {
        name: "enabled",
        kind: "bool/i32",
        description: "Nonzero precomposes backing/frame/text as an isolated group.",
    },
];
const GRAPH90_SET_WINDOW_VALID_REGION_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "window",
        kind: "window handle",
        description: "Target CDspObjWindow.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "i32 coordinate",
        description: "Valid-region left coordinate.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32 coordinate",
        description: "Valid-region top coordinate.",
    },
    NativeParameterSpec {
        name: "width",
        kind: "positive i32 extent",
        description: "Stored as inclusive right=x+width-1 after bounds validation.",
    },
    NativeParameterSpec {
        name: "height",
        kind: "positive i32 extent",
        description: "Stored as inclusive bottom=y+height-1 after bounds validation.",
    },
];
const GRAPH90_GET_WINDOW_VALID_REGION_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "out_rect",
        kind: "BP pointer to four i32 values",
        description: "Receives left, top, right, bottom from CDspObjWindow+416..+428.",
    },
    NativeParameterSpec {
        name: "window",
        kind: "window handle",
        description: "Target CDspObjWindow.",
    },
];
const GRAPH90_START_WINDOW_MESSAGE_PROCEDURE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "window",
        kind: "window handle",
        description: "Message destination; target requires a configured font.",
    },
    NativeParameterSpec {
        name: "text",
        kind: "Shift-JIS message string",
        description: "Parsed by CProcDspMsg.",
    },
    NativeParameterSpec {
        name: "procedure_value",
        kind: "i32",
        description: "Forwarded through the target procedure configuration virtual path; concrete field meaning remains open.",
    },
    NativeParameterSpec {
        name: "completion_control",
        kind: "i32",
        description: "Stored at CProcDspMsg+0x30.",
    },
    NativeParameterSpec {
        name: "end_wait_policy",
        kind: "i32",
        description: "Stored at CProcDspMsg+0x38.",
    },
    NativeParameterSpec {
        name: "allow_high_bit_input",
        kind: "bool/i32",
        description: "Stored at CProcDspMsg+0x3C; controls bit-31 input acceptance.",
    },
];
const GRAPH90_CONFIGURE_CARET_FRAMES_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "frame_count",
        kind: "i32 count",
        description: "Count of caret frame entries; count <=1 clears or leaves no frame table.",
    },
    NativeParameterSpec {
        name: "bitmap_table",
        kind: "BP pointer to bitmap handles",
        description: "Each entry is copied into an internal frame; -1 creates an empty frame.",
    },
];
const GRAPH90_CARET_DELAY_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "delay_ms",
    kind: "i32 milliseconds",
    description: "Caret animation frame interval consumed by CProcDspMsg.",
}];
const GRAPH90_CARET_POSITION_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "mode",
        kind: "i32 coordinate mode",
        description: "Mode 1 is absolute; other values are relative to the current window message position.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "i32 coordinate/offset",
        description: "Caret X or relative X offset.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32 coordinate/offset",
        description: "Caret Y or relative Y offset.",
    },
];
const GRAPH90_SHADOW_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "x_percent",
        kind: "i32 0..=100",
        description: "Horizontal offset as a percentage of font height.",
    },
    NativeParameterSpec {
        name: "y_percent",
        kind: "i32 0..=100",
        description: "Vertical offset as a percentage of font height.",
    },
    NativeParameterSpec {
        name: "concentration",
        kind: "i32 0..=256",
        description: "Target uses 256-concentration in the glyph blend path.",
    },
];
const GRAPH90_GLYPH_STRIP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap handle",
        description: "Horizontal source strip; width must divide evenly by character_count.",
    },
    NativeParameterSpec {
        name: "character_count",
        kind: "i32 0..=255",
        description: "Positive values register slices at 0xFF01 onward; nonpositive clears the fixed atlas.",
    },
];

const GRAPH90_START_ITEM_SELECTION_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "window",
        kind: "window handle",
        description: "Destination CDspObjWindow.",
    },
    NativeParameterSpec {
        name: "item_count",
        kind: "i32 1..=16",
        description: "Number of selectable strings.",
    },
    NativeParameterSpec {
        name: "item_strings",
        kind: "BP pointer to string table",
        description: "Target copies item_count strings.",
    },
    NativeParameterSpec {
        name: "column_count",
        kind: "i32 1..=16",
        description: "Grid column count.",
    },
    NativeParameterSpec {
        name: "layout_mode",
        kind: "i32",
        description: "Forwarded to the item-grid layout helper.",
    },
    NativeParameterSpec {
        name: "text_style",
        kind: "i32",
        description: "Forwarded to the item text renderer.",
    },
    NativeParameterSpec {
        name: "initial_selection",
        kind: "i32 index",
        description: "Must be inside the item range.",
    },
    NativeParameterSpec {
        name: "cancel_enabled",
        kind: "bool/i32",
        description: "Controls whether cancellation input bits are masked.",
    },
];
const GRAPH90_DRAW_ITEM_SELECTION_GRID_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "window",
        kind: "window handle",
        description: "Destination CDspObjWindow.",
    },
    NativeParameterSpec {
        name: "item_count",
        kind: "i32 1..=16",
        description: "Number of strings.",
    },
    NativeParameterSpec {
        name: "item_strings",
        kind: "BP pointer to string table",
        description: "Target copies and draws these strings.",
    },
    NativeParameterSpec {
        name: "column_count",
        kind: "i32 1..=16",
        description: "Grid column count.",
    },
    NativeParameterSpec {
        name: "layout_mode",
        kind: "i32",
        description: "Item layout selector.",
    },
    NativeParameterSpec {
        name: "text_style",
        kind: "i32",
        description: "Fallback item text style.",
    },
];
const GRAPH90_START_ITEM_SELECTION_EX_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "window",
        kind: "window handle",
        description: "Destination CDspObjWindow.",
    },
    NativeParameterSpec {
        name: "item_count",
        kind: "i32 1..=16",
        description: "Number of selectable strings.",
    },
    NativeParameterSpec {
        name: "item_strings",
        kind: "BP pointer to string table",
        description: "Target copies item_count strings.",
    },
    NativeParameterSpec {
        name: "column_count",
        kind: "i32 1..=16",
        description: "Grid column count.",
    },
    NativeParameterSpec {
        name: "layout_mode",
        kind: "i32",
        description: "Forwarded to the base item-grid helper.",
    },
    NativeParameterSpec {
        name: "text_style",
        kind: "i32",
        description: "Forwarded to the base item-grid helper.",
    },
    NativeParameterSpec {
        name: "initial_selection",
        kind: "i32 index",
        description: "Initial selected item.",
    },
    NativeParameterSpec {
        name: "cancel_enabled",
        kind: "bool/i32",
        description: "Cancellation gate.",
    },
    NativeParameterSpec {
        name: "primary_bitmap",
        kind: "bitmap handle or -1",
        description: "Optional first overlay bitmap.",
    },
    NativeParameterSpec {
        name: "primary_offset_x",
        kind: "i32",
        description: "First overlay X offset.",
    },
    NativeParameterSpec {
        name: "primary_offset_y",
        kind: "i32",
        description: "First overlay Y offset.",
    },
    NativeParameterSpec {
        name: "secondary_bitmap",
        kind: "bitmap handle or -1",
        description: "Optional second overlay bitmap.",
    },
    NativeParameterSpec {
        name: "secondary_offset_x",
        kind: "i32",
        description: "Second overlay X offset.",
    },
    NativeParameterSpec {
        name: "secondary_offset_y",
        kind: "i32",
        description: "Second overlay Y offset.",
    },
];
const GRAPH90_SELECTION_HIGHLIGHT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "style_a",
        kind: "i32 style/color",
        description: "First alternating selected-item style.",
    },
    NativeParameterSpec {
        name: "style_b",
        kind: "i32 style/color",
        description: "Second alternating selected-item style.",
    },
];
const GRAPH90_SELECTION_INPUT_MASK_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "input_mask",
    kind: "i32 bit mask",
    description: "Process-global CProcSelectItem input mask.",
}];
const GRAPH90_SELECTION_NAVIGATION_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "alternate_key_mode",
        kind: "bool/i32",
        description: "Selects the alternate directional key table.",
    },
    NativeParameterSpec {
        name: "cursor_move_arg1",
        kind: "i32",
        description: "First cursor-centering helper parameter.",
    },
    NativeParameterSpec {
        name: "cursor_move_arg2",
        kind: "i32",
        description: "Second cursor-centering helper parameter.",
    },
    NativeParameterSpec {
        name: "cursor_move_arg3",
        kind: "i32",
        description: "Third stored navigation parameter.",
    },
];
const GRAPH90_SELECTION_COLUMN_LAYOUT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "window",
        kind: "window handle",
        description: "Window whose per-item text colors are updated.",
    },
    NativeParameterSpec {
        name: "layout",
        kind: "BP pointer to 16 i32 values or null",
        description: "Target copies exactly 16 RGB DWORDs and toggles the per-item-color gate.",
    },
];
const GRAPH90_INTERACTIVE_POLL_GATE_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "enabled",
    kind: "bool/i32",
    description: "Stored in both target input-poll gate globals.",
}];
const GRAPH90_START_ICON_SELECTION_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "window",
        kind: "window handle",
        description: "Destination CDspObjWindow.",
    },
    NativeParameterSpec {
        name: "icon_count",
        kind: "i32 1..=64",
        description: "Number of icon records.",
    },
    NativeParameterSpec {
        name: "icon_records",
        kind: "BP pointer",
        description: "Base records are 16 bytes; Ex records are 64 bytes.",
    },
    NativeParameterSpec {
        name: "input_table",
        kind: "optional BP pointer",
        description: "Optional per-icon input descriptor table.",
    },
    NativeParameterSpec {
        name: "input_mode",
        kind: "i32 0..=3",
        description: "Constructor mode selected through hidden ECX.",
    },
    NativeParameterSpec {
        name: "procedure_state",
        kind: "i32",
        description: "Stored in the target procedure state field at +1612.",
    },
    NativeParameterSpec {
        name: "extra_image_enabled",
        kind: "bool/i32",
        description: "Enables the optional extra image/mask path.",
    },
];
const GRAPH90_DRAW_ICON_BATCH_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "window",
        kind: "window handle",
        description: "Destination CDspObjWindow.",
    },
    NativeParameterSpec {
        name: "icon_count",
        kind: "i32 1..=64",
        description: "Number of records.",
    },
    NativeParameterSpec {
        name: "records",
        kind: "BP pointer",
        description: "Base records are 16 bytes; extended records are projected from 64-byte strides.",
    },
];
const GRAPH90_ICON_LAYOUT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "window",
        kind: "window handle",
        description: "Destination CDspObjWindow.",
    },
    NativeParameterSpec {
        name: "descriptor",
        kind: "BP descriptor pointer",
        description: "Nested group/item table copied by the target parser.",
    },
];
const GRAPH90_ICON_PROCESSOR_HANDLE_PARAMETER: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "processor",
    kind: "DCIPIcon registry handle",
    description: "Opaque handle resolved through the target dynamic object registry.",
}];
/// Graph90:BA / ConfigureIconInputProcessor compact descriptor ABI.
///
/// Target `sub_47EF00 -> sub_46C8E0 -> sub_46C750 -> sub_447C10` copies a
/// 32-byte root, 52-byte group records and 60-byte item records. The compact
/// group fields used by the input state machine are:
/// - group+0x08: configure-time/current item index; -1 means none. If the
///   referenced item's +0x00 enable/depth field is zero, configure invalidates
///   this field back to -1.
/// - group+0x0C: group selection enabled; `sub_449A60` requires nonzero.
/// - group+0x10: pointer movement may change this group's current item and
///   produce 0x10000004.
/// - group+0x14: while mouse-left is already held, `sub_448690` may inject
///   action bit 1 from `sub_46E490()`. It is not a fresh-click permission bit.
/// - group+0x18: selection-exclusion key; equal non--1 keys are mutually
///   cleared by `sub_449D60` when another group becomes current.
///
/// The compact item visual
/// resources consumed by `sub_4499F0` are distinct state slots:
/// - item+0x0C: normal/idle bitmap resource id, or -1 when absent.
/// - item+0x10: per-group current/selected bitmap resource id, or -1.
/// - item+0x14: pointer-hover bitmap resource id, or -1.
/// - item+0x18: auxiliary resource used by selector 4 mutation paths; it is not
///   the pointer-hover bitmap.
/// Pointer hover is runtime DCIPIcon+0x74 (`this[29]`), raw hit is +0x80
/// (`this[32]`), and action state is +0x68/+0x6C/+0x70. None of those runtime
/// fields may be written back into the BP descriptor.
const GRAPH90_CONFIGURE_ICON_PROCESSOR_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "processor",
        kind: "DCIPIcon registry handle",
        description: "Must resolve to the base DCIPIcon variant; invalid/non-base handles are rejected.",
    },
    NativeParameterSpec {
        name: "descriptor",
        kind: "BP compact descriptor pointer",
        description: "32-byte root -> 52-byte groups -> 60-byte items. Root +0x14 suppresses physical action bits and +0x18 selects action-map mode 0..7. Group: +0x08 current item (-1 none), +0x0C selection enable, +0x10 pointer-selection enable, +0x14 held mouse-left action reinjection, +0x18 exclusion key. Item bitmap slots: +0x0C normal, +0x10 group-current, +0x14 pointer-hover, +0x18 auxiliary.",
    },
];

/// Graph90:BC / GetIconInputState output ABI.
/// `sub_46CC00 -> sub_4484C0` writes exactly six i32 values:
/// `[running, group, item, state, local_x, local_y]`. `running` is native
/// DCIPIcon+0x30, set to 1 by configure and cleared by `sub_4485A0` when the
/// per-tick handler completes an action. `group`/`item` are -1 when no item is
/// attached. `state` is the action state (confirmed 0/1 paths). When state is
/// nonzero, `sub_44A6F0` derives item-local pointer coordinates for words 4/5.
/// Pure hover/current-selection changes do not write words 1..=5: target hover
/// lives at DCIPIcon+0x74/+0x80 and group current items live in the group table.
/// Treating hover as a BC action target advances scripts down activation paths.
const GRAPH90_ICON_STATE_OUTPUT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "output",
        kind: "writable BP pointer to 6 x i32 (24 bytes)",
        description: "Receives [running, group, item, state, local_x, local_y]. group/item use -1 for no item; state has confirmed 0/1 paths.",
    },
    NativeParameterSpec {
        name: "processor",
        kind: "DCIPIcon registry handle",
        description: "Source input processor. Invalid handle returns 0 and leaves output untouched.",
    },
];

/// Graph90:BD / GetIconInputCurrentGroup output ABI.
const GRAPH90_ICON_CURRENT_GROUP_OUTPUT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "output",
        kind: "writable BP pointer to i32",
        description: "Receives DCIPIcon+0x3C: -1 for no current group, otherwise a descriptor group index.",
    },
    NativeParameterSpec {
        name: "processor",
        kind: "DCIPIcon registry handle",
        description: "Source input processor. Invalid handle returns 0.",
    },
];

/// Graph90:BE / GetIconInputSelections output ABI.
const GRAPH90_ICON_SELECTION_OUTPUT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "output",
        kind: "writable BP pointer to group_count x i32",
        description: "Receives each 52-byte group's +0x08 selected/current item index in group order; -1 means no item selected.",
    },
    NativeParameterSpec {
        name: "processor",
        kind: "DCIPIcon registry handle",
        description: "Source input processor. Invalid handle returns 0.",
    },
];

/// Graph90:BF / PopIconInputEvent ABI.
///
/// `sub_47F000 -> sub_46CC70 -> sub_448560` copies exactly one FIFO head
/// `[event_code,payload,parameter]`, then `sub_44A220` removes that node. A
/// valid empty queue writes `[0,0,0]`; repeated BF polls cannot recreate an
/// event that has already been popped.
///
/// Confirmed event producers relevant to icon input:
/// - 0x10000001: raw hit changed. Base `sub_448690` compares
///   `sub_4495C0(...,0,0)` with DCIPIcon+0x80; words 1/2 are group/item or
///   -1/-1 on leave.
/// - 0x10000002: pointer-current/hover item changed. Base `sub_448690` calls
///   `sub_449760 -> vtable+0x1C (sub_4497A0)` and, on change, vtable+0x10
///   (`sub_448670`) appends this record. `payload` is -1 on leave or packed
///   LOWORD=item/HIWORD=group; `parameter` is 1 iff compact item+0x14 has a
///   hover bitmap, else 0. DCIPIconEx `sub_44B970` suppresses a valid-item 02
///   record when item flags bit 0x4 is set.
/// - 0x10000003: current-group navigation changed; payload is the group index
///   and parameter is -1/1 for previous/next navigation.
/// - 0x10000004: current-item selection/navigation changed; pointer-driven
///   changes require group+0x10 and use packed group/item with parameter 0.
/// - 0x10000005: current-item navigation boundary/fallback notification from
///   the target navigation branches. It is not a mouse-click event.
/// - 0x10000006: DCIPIconEx state transition from `sub_44C170/sub_44C230`;
///   parameter is the transition state (confirmed 0/1). Base DCIPIcon's
///   `sub_449FA0/sub_44A000` only update BC state and do not append 06.
/// - 0x10000007: DCIPIconEx action-position notification. `sub_44B9E0` emits
///   payload=-1 plus packed processor-local x/y for a no-item action; nonzero
///   extended item-state paths may emit packed group/item plus local x/y.
const GRAPH90_POP_ICON_INPUT_EVENT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "output",
        kind: "writable BP pointer to 3 x i32 (12 bytes)",
        description: "Receives [event_code, payload, parameter]. Valid empty queue writes [0,0,0]. Confirmed codes include 01 raw-hit change, 02 pointer-current change, 03 group navigation, 04/05 item navigation, and extended 06 state / 07 action-position. Packed item payload uses LOWORD=item, HIWORD=group; -1 means no item.",
    },
    NativeParameterSpec {
        name: "processor",
        kind: "i32 DCIPIcon registry handle",
        description: "Any i32 is accepted at the ABI boundary, but it must resolve to a live DCIPIcon/DCIPIconEx registry object. Invalid handle returns 0 and does not perform the queue pop; valid handle returns 1.",
    },
];

const GRAPH90_LOAD_BG_BITMAP_RESOURCE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap handle",
        description: "Destination bitmap slot.",
    },
    NativeParameterSpec {
        name: "namespace",
        kind: "string",
        description: "Archive or BG resource namespace.",
    },
    NativeParameterSpec {
        name: "name",
        kind: "string",
        description: "BG resource name.",
    },
];
const GRAPH90_FLIP_BITMAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "bitmap handle",
        description: "Receives the mirrored pixels.",
    },
    NativeParameterSpec {
        name: "source",
        kind: "bitmap handle",
        description: "Source bitmap.",
    },
    NativeParameterSpec {
        name: "direction",
        kind: "0 horizontal / 1 vertical",
        description: "Target rejects any other value.",
    },
];
const GRAPH90_BITMAP_PAIR_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "bitmap handle",
        description: "Destination bitmap.",
    },
    NativeParameterSpec {
        name: "source",
        kind: "bitmap handle",
        description: "Source bitmap.",
    },
];
const GRAPH90_IMPORT_EXTERNAL_IMAGE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "writable BP pointer",
        description: "Receives the target GDI+ image descriptor.",
    },
    NativeParameterSpec {
        name: "filename",
        kind: "filesystem string",
        description: "External image path.",
    },
    NativeParameterSpec {
        name: "pixel_mode",
        kind: "-1,1,2,3",
        description: "Automatic or explicit target pixel-format request.",
    },
];
const GRAPH90_SAVE_BITMAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "filename",
        kind: "filesystem string",
        description: "Output path.",
    },
    NativeParameterSpec {
        name: "format",
        kind: "0..=4",
        description: "BMP, JPEG, GIF, TIFF, or PNG.",
    },
    NativeParameterSpec {
        name: "quality",
        kind: "i32",
        description: "GDI+ encoder parameter used by applicable formats.",
    },
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap handle",
        description: "Source bitmap.",
    },
];
const GRAPH90_REGISTER_BG_RESOURCE_DATA_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "namespace",
        kind: "string",
        description: "First lower-cased cache key.",
    },
    NativeParameterSpec {
        name: "name",
        kind: "string",
        description: "Second lower-cased cache key.",
    },
    NativeParameterSpec {
        name: "data",
        kind: "readable BP pointer",
        description: "BG byte buffer.",
    },
    NativeParameterSpec {
        name: "size",
        kind: "i32 byte count",
        description: "Number of bytes copied into the native cache.",
    },
];
const GRAPH90_LOAD_CACHED_BG_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap handle",
        description: "Destination bitmap.",
    },
    NativeParameterSpec {
        name: "namespace",
        kind: "string",
        description: "First cache/archive key.",
    },
    NativeParameterSpec {
        name: "name",
        kind: "string",
        description: "Second cache/archive key.",
    },
    NativeParameterSpec {
        name: "consume_cached",
        kind: "bool/i32",
        description: "Nonzero consumes cached bytes after lookup.",
    },
];
const GRAPH90_TRANSFORM_BITMAP_GENERAL_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "arg0",
        kind: "i32",
        description: "Source-order target argument 0.",
    },
    NativeParameterSpec {
        name: "arg1",
        kind: "i32",
        description: "Source-order target argument 1.",
    },
    NativeParameterSpec {
        name: "arg2",
        kind: "i32",
        description: "Source-order target argument 2.",
    },
    NativeParameterSpec {
        name: "arg3",
        kind: "i32",
        description: "Source-order target argument 3.",
    },
    NativeParameterSpec {
        name: "arg4",
        kind: "i32",
        description: "Source-order target argument 4.",
    },
    NativeParameterSpec {
        name: "arg5",
        kind: "i32",
        description: "Source-order target argument 5.",
    },
    NativeParameterSpec {
        name: "arg6",
        kind: "i32",
        description: "Source-order target argument 6.",
    },
    NativeParameterSpec {
        name: "arg7",
        kind: "i32",
        description: "Source-order target argument 7.",
    },
    NativeParameterSpec {
        name: "arg8",
        kind: "i32",
        description: "Source-order target argument 8.",
    },
    NativeParameterSpec {
        name: "arg9",
        kind: "i32",
        description: "Source-order target argument 9.",
    },
    NativeParameterSpec {
        name: "arg10",
        kind: "i32",
        description: "Source-order target argument 10.",
    },
    NativeParameterSpec {
        name: "arg11",
        kind: "i32",
        description: "Source-order target argument 11.",
    },
    NativeParameterSpec {
        name: "arg12",
        kind: "i32",
        description: "Source-order target argument 12.",
    },
    NativeParameterSpec {
        name: "arg13",
        kind: "i32",
        description: "Source-order target argument 13.",
    },
    NativeParameterSpec {
        name: "arg14",
        kind: "i32",
        description: "Source-order target argument 14.",
    },
    NativeParameterSpec {
        name: "arg15",
        kind: "i32",
        description: "Source-order target argument 15.",
    },
    NativeParameterSpec {
        name: "arg16",
        kind: "i32",
        description: "Source-order target argument 16.",
    },
    NativeParameterSpec {
        name: "arg17",
        kind: "i32",
        description: "Source-order target argument 17.",
    },
];
const GRAPH90_REGISTER_TONE_CURVE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "identifier",
        kind: "i32",
        description: "Tone-curve registry key.",
    },
    NativeParameterSpec {
        name: "control_points",
        kind: "optional BP pointer to 6 i32",
        description: "Three (x,y) control points; null removes the identifier.",
    },
];
const GRAPH90_APPLY_TONE_CURVE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "bitmap handle",
        description: "Destination bitmap.",
    },
    NativeParameterSpec {
        name: "source",
        kind: "bitmap handle",
        description: "Source bitmap.",
    },
    NativeParameterSpec {
        name: "color_before",
        kind: "packed RGB",
        description: "First color-film endpoint.",
    },
    NativeParameterSpec {
        name: "amount_before",
        kind: "mix level",
        description: "First color-film/curve mix amount.",
    },
    NativeParameterSpec {
        name: "tone_curve",
        kind: "tone-curve id",
        description: "Registered LUT identifier.",
    },
    NativeParameterSpec {
        name: "color_after",
        kind: "packed RGB",
        description: "Second color-film endpoint.",
    },
    NativeParameterSpec {
        name: "effect_mode",
        kind: "i32",
        description: "Target color/monochrome effect selector.",
    },
    NativeParameterSpec {
        name: "amount_after",
        kind: "mix level",
        description: "Final effect amount.",
    },
];
const GRAPH90_ENCODE_BITMAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "optional writable BP pointer",
        description: "Receives encoded bytes when nonnull.",
    },
    NativeParameterSpec {
        name: "size_out",
        kind: "writable BP pointer",
        description: "Receives encoded byte count.",
    },
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap handle",
        description: "Source bitmap.",
    },
    NativeParameterSpec {
        name: "format",
        kind: "0 or 1",
        description: "Target proprietary encoder selector.",
    },
    NativeParameterSpec {
        name: "parameter",
        kind: "0..=100 for format 1",
        description: "Secondary codec quality/parameter.",
    },
];
const GRAPH90_OPEN_DIRECTSHOW_MOVIE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "movie_path",
        kind: "Shift-JIS path/resource",
        description: "Target DirectShow movie path.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "i32",
        description: "Popped and ignored by the target public wrapper.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32",
        description: "Popped and ignored by the target public wrapper.",
    },
    NativeParameterSpec {
        name: "width",
        kind: "positive i32",
        description: "Validated movie window width.",
    },
    NativeParameterSpec {
        name: "height",
        kind: "positive i32",
        description: "Validated movie window height.",
    },
];
const GRAPH90_MOVIE_VOLUME_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "volume",
    kind: "0..=128",
    description: "Mapped to the target DirectShow -10000..0 dB range.",
}];
const GRAPH90_LOAD_BURIKO_MOVIE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle_out",
        kind: "writable BP pointer to i32",
        description: "Receives the monotonic BF_Movie resource handle.",
    },
    NativeParameterSpec {
        name: "metadata_out",
        kind: "writable BP pointer to five i32",
        description: "Receives header DWORDs at offsets 20, 24, 32, 36 and 40.",
    },
    NativeParameterSpec {
        name: "archive",
        kind: "optional Shift-JIS archive/path",
        description: "Optional resource namespace passed to DCProcLoadBurikoMV.",
    },
    NativeParameterSpec {
        name: "resource",
        kind: "Shift-JIS resource name",
        description: "BF_Movie resource loaded by the cooperative procedure.",
    },
];
const GRAPH90_BURIKO_MOVIE_HANDLE_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "movie",
    kind: "BF_Movie resource handle",
    description: "Handle from Graph90:F4/F7.",
}];
const GRAPH90_DECODE_BURIKO_MOVIE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "bitmap handle",
        description: "Must match movie width, height and format.",
    },
    NativeParameterSpec {
        name: "movie",
        kind: "BF_Movie resource handle",
        description: "Source resource.",
    },
    NativeParameterSpec {
        name: "frame_index",
        kind: "nonnegative frame index",
        description: "Must be less than header DWORD at offset 40.",
    },
];
const GRAPH90_ATTACH_BURIKO_MOVIE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle_out",
        kind: "writable BP pointer to i32",
        description: "Receives the attached shared child handle.",
    },
    NativeParameterSpec {
        name: "source",
        kind: "BF_Movie resource handle",
        description: "Source; only one attached child is permitted.",
    },
];
const GRAPH90_SPRITE_TARGET_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "sprite",
    kind: "0x80000000 tagged Sprite handle",
    description: "Target CDspObjSprite registered with the input manager.",
}];
const GRAPH90_SPRITE_TARGET_NUMBER_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "target_number",
    kind: "i32",
    description: "Monotonic identifier assigned by Graph90:FA.",
}];

const GRAPH91_CACHE_BINARY_RESOURCE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "namespace",
        kind: "Shift-JIS string",
        description: "First key in the native graph cache.",
    },
    NativeParameterSpec {
        name: "name",
        kind: "Shift-JIS string",
        description: "Second key in the native graph cache.",
    },
    NativeParameterSpec {
        name: "source",
        kind: "readable BP pointer",
        description: "Source bytes in VM memory.",
    },
    NativeParameterSpec {
        name: "size",
        kind: "byte count",
        description: "Number of source bytes copied.",
    },
];
const GRAPH91_DISPLAY_OFFSET_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "x",
        kind: "i32",
        description: "Global display-object X offset.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32",
        description: "Global display-object Y offset.",
    },
];
const GRAPH91_BOOLEAN_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "enabled",
    kind: "i32 boolean",
    description: "Process-global native gate.",
}];
const GRAPH91_GLYPH_COVERAGE_MODE_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "mode",
    kind: "0 or 1",
    description: "Linear or sine-shaped glyph coverage conversion.",
}];
const GRAPH91_FONT_TRANSFORM_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "name",
        kind: "Shift-JIS font name",
        description: "Case-insensitive transform registry key.",
    },
    NativeParameterSpec {
        name: "scale_x",
        kind: "0 or 16.16 in 1.0..=2.0",
        description: "Horizontal native glyph scale.",
    },
    NativeParameterSpec {
        name: "scale_y",
        kind: "0 or 16.16 in 1.0..=2.0",
        description: "Vertical native glyph scale.",
    },
    NativeParameterSpec {
        name: "offset_x",
        kind: "i32",
        description: "Horizontal glyph adjustment.",
    },
    NativeParameterSpec {
        name: "offset_y",
        kind: "i32",
        description: "Vertical glyph adjustment.",
    },
];
const GRAPH91_NATIVE_FONT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "font_number",
        kind: "native face index",
        description: "Numbered target font face.",
    },
    NativeParameterSpec {
        name: "size",
        kind: "4..=200",
        description: "Native font height.",
    },
    NativeParameterSpec {
        name: "width_percent",
        kind: "25..=200",
        description: "Native requested width percentage.",
    },
    NativeParameterSpec {
        name: "bold",
        kind: "i32",
        description: "Bold value included in the target font cache key.",
    },
    NativeParameterSpec {
        name: "field_56",
        kind: "i32",
        description: "Target font-record field at offset 0x38.",
    },
    NativeParameterSpec {
        name: "field_60",
        kind: "i32",
        description: "Target font-record field at offset 0x3C.",
    },
];
const GRAPH91_AFFINE_MAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "bitmap",
        kind: "format-4 bitmap",
        description: "Destination displacement map.",
    },
    NativeParameterSpec {
        name: "center_x",
        kind: "i32",
        description: "Horizontal origin.",
    },
    NativeParameterSpec {
        name: "center_y",
        kind: "i32",
        description: "Vertical origin.",
    },
    NativeParameterSpec {
        name: "span_x",
        kind: "i32",
        description: "Horizontal affine span.",
    },
    NativeParameterSpec {
        name: "span_y",
        kind: "i32",
        description: "Vertical affine span.",
    },
];
const GRAPH91_RANDOM_MAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "bitmap",
        kind: "format-4 bitmap",
        description: "Destination displacement map.",
    },
    NativeParameterSpec {
        name: "amplitude",
        kind: "i32",
        description: "Inclusive random offset radius.",
    },
];
const GRAPH91_RIPPLE_MAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "bitmap",
        kind: "format-4 bitmap",
        description: "Destination displacement map.",
    },
    NativeParameterSpec {
        name: "center_x",
        kind: "i32",
        description: "Ripple center X.",
    },
    NativeParameterSpec {
        name: "center_y",
        kind: "i32",
        description: "Ripple center Y.",
    },
    NativeParameterSpec {
        name: "period",
        kind: "nonzero i32",
        description: "Cosine radial period.",
    },
    NativeParameterSpec {
        name: "phase",
        kind: "i32",
        description: "Radial phase offset.",
    },
    NativeParameterSpec {
        name: "amplitude",
        kind: "i32",
        description: "Scalar-field amplitude.",
    },
];
const GRAPH91_FIVE_MAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "bitmap",
        kind: "format-4 bitmap",
        description: "Destination displacement map.",
    },
    NativeParameterSpec {
        name: "center_x",
        kind: "i32",
        description: "Effect center X.",
    },
    NativeParameterSpec {
        name: "center_y",
        kind: "i32",
        description: "Effect center Y.",
    },
    NativeParameterSpec {
        name: "parameter",
        kind: "i32",
        description: "Angle, curvature or strength according to selector.",
    },
    NativeParameterSpec {
        name: "radius_or_perspective",
        kind: "i32",
        description: "Radius or perspective according to selector.",
    },
];
const GRAPH91_SINE_MAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "bitmap",
        kind: "format-4 bitmap",
        description: "Destination displacement map.",
    },
    NativeParameterSpec {
        name: "x_period",
        kind: "i32",
        description: "Period for row-driven X displacement.",
    },
    NativeParameterSpec {
        name: "x_phase",
        kind: "i32",
        description: "Phase for row-driven X displacement.",
    },
    NativeParameterSpec {
        name: "x_amplitude",
        kind: "i32",
        description: "Amplitude for row-driven X displacement.",
    },
    NativeParameterSpec {
        name: "y_period",
        kind: "i32",
        description: "Period for column-driven Y displacement.",
    },
    NativeParameterSpec {
        name: "y_phase",
        kind: "i32",
        description: "Phase for column-driven Y displacement.",
    },
    NativeParameterSpec {
        name: "y_amplitude",
        kind: "i32",
        description: "Amplitude for column-driven Y displacement.",
    },
];
const GRAPH91_RADIAL_WARP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "bitmap",
        kind: "format-4 bitmap",
        description: "Destination displacement map.",
    },
    NativeParameterSpec {
        name: "source_origin_x",
        kind: "i32",
        description: "Source origin X.",
    },
    NativeParameterSpec {
        name: "source_origin_y",
        kind: "i32",
        description: "Source origin Y.",
    },
    NativeParameterSpec {
        name: "warp_center_x",
        kind: "i32",
        description: "Warp center X.",
    },
    NativeParameterSpec {
        name: "warp_center_y",
        kind: "i32",
        description: "Warp center Y.",
    },
    NativeParameterSpec {
        name: "radius_bias",
        kind: "i32",
        description: "Added to the distance-derived warp radius.",
    },
];
const GRAPH91_RECT_COMPOSITE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "bitmap handle",
        description: "Destination bitmap.",
    },
    NativeParameterSpec {
        name: "destination_x",
        kind: "i32",
        description: "Destination rectangle X.",
    },
    NativeParameterSpec {
        name: "destination_y",
        kind: "i32",
        description: "Destination rectangle Y.",
    },
    NativeParameterSpec {
        name: "source",
        kind: "bitmap handle",
        description: "Source bitmap.",
    },
    NativeParameterSpec {
        name: "source_x",
        kind: "i32",
        description: "Source rectangle X.",
    },
    NativeParameterSpec {
        name: "source_y",
        kind: "i32",
        description: "Source rectangle Y.",
    },
    NativeParameterSpec {
        name: "mode_or_key",
        kind: "i32",
        description: "Target format-dependent mode or color key.",
    },
    NativeParameterSpec {
        name: "width",
        kind: "nonzero i32",
        description: "Rectangle width.",
    },
    NativeParameterSpec {
        name: "height",
        kind: "nonzero i32",
        description: "Rectangle height.",
    },
    NativeParameterSpec {
        name: "alpha",
        kind: "0..=256",
        description: "Native blend parameter.",
    },
    NativeParameterSpec {
        name: "source_alpha_gate",
        kind: "i32 boolean",
        description: "Whether source alpha participates.",
    },
];
const GRAPH91_REPLACE_RGB_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "format-2 bitmap",
        description: "Destination bitmap.",
    },
    NativeParameterSpec {
        name: "source",
        kind: "format-2 bitmap",
        description: "Source alpha provider.",
    },
    NativeParameterSpec {
        name: "packed_rgb",
        kind: "0x00RRGGBB",
        description: "Replacement RGB value.",
    },
];
const GRAPH91_CONCENTRATE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "bitmap handle",
        description: "Destination bitmap.",
    },
    NativeParameterSpec {
        name: "source",
        kind: "bitmap handle",
        description: "Source bitmap.",
    },
    NativeParameterSpec {
        name: "concentration_x",
        kind: "0..=0x10000",
        description: "Horizontal 16.16 concentration.",
    },
    NativeParameterSpec {
        name: "concentration_y",
        kind: "0..=0x10000",
        description: "Vertical 16.16 concentration.",
    },
    NativeParameterSpec {
        name: "brightness_attenuation",
        kind: "0..=256",
        description: "Target brightness attenuation.",
    },
];
const GRAPH91_SCALE_TRUE_COLOR_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "bitmap handle",
        description: "Destination bitmap allocated by the handler.",
    },
    NativeParameterSpec {
        name: "source",
        kind: "format-1 or format-2 bitmap",
        description: "True-color source bitmap.",
    },
    NativeParameterSpec {
        name: "scale_x",
        kind: "16.16",
        description: "Horizontal scaling factor.",
    },
    NativeParameterSpec {
        name: "scale_y",
        kind: "16.16",
        description: "Vertical scaling factor.",
    },
    NativeParameterSpec {
        name: "interpolation",
        kind: "i32 boolean",
        description: "Target resampling mode selector.",
    },
];
const GRAPH91_PROCESS_BITMAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "bitmap handle",
        description: "Destination bitmap.",
    },
    NativeParameterSpec {
        name: "source",
        kind: "bitmap handle",
        description: "Source bitmap.",
    },
    NativeParameterSpec {
        name: "process_type",
        kind: "0..=5",
        description: "Target pixel-processing selector.",
    },
    NativeParameterSpec {
        name: "parameter",
        kind: "i32",
        description: "Process-specific value.",
    },
    NativeParameterSpec {
        name: "alpha",
        kind: "0..=256",
        description: "Native blend parameter.",
    },
];
const GRAPH91_GRAYSCALE_MASK_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "target",
        kind: "format-2 or format-3 bitmap",
        description: "Target bitmap.",
    },
    NativeParameterSpec {
        name: "grayscale_source",
        kind: "format-3 bitmap",
        description: "Grayscale mask bitmap.",
    },
    NativeParameterSpec {
        name: "offset_y",
        kind: "i32",
        description: "Vertical mask offset in source ABI order.",
    },
    NativeParameterSpec {
        name: "offset_x",
        kind: "i32",
        description: "Horizontal mask offset in source ABI order.",
    },
];
const GRAPH91_CLONE_BITMAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "bitmap handle",
        description: "Destination bitmap.",
    },
    NativeParameterSpec {
        name: "source",
        kind: "bitmap handle",
        description: "Source bitmap cloned in full.",
    },
];

const GRAPH91_OBJECT_BOOLEAN_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "object",
        kind: "CDspObj handle",
        description: "Target display object.",
    },
    NativeParameterSpec {
        name: "suppressed",
        kind: "i32 boolean",
        description: "Stored at CDspObj+0x0C and tested inversely by sub_41AE30.",
    },
];
const GRAPH91_OBJECT_FIXED_POSITION_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "object",
        kind: "CDspObj handle",
        description: "Target display object.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "signed 16.16",
        description: "Fixed-point X written by vtable slot +60.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "signed 16.16",
        description: "Fixed-point Y written by vtable slot +60.",
    },
    NativeParameterSpec {
        name: "z",
        kind: "signed 16.16",
        description: "Fixed-point Z written at CDspObj+0x54. sub_41B4C0 adds this object's three vector banks; parent/member motion is already materialized by eager setter propagation before mode-5 projection consumes the resolved Z.",
    },
];
const GRAPH91_OBJECT_VECTOR_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "object",
        kind: "CDspObj handle",
        description: "Target display object.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "signed 16.16",
        description: "Vector X component.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "signed 16.16",
        description: "Vector Y component.",
    },
    NativeParameterSpec {
        name: "z",
        kind: "signed 16.16",
        description: "Vector Z component.",
    },
];
const GRAPH91_GET_OBJECT_PROPERTY_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "output",
        kind: "writable BP pointer to i32",
        description: "Receives the subclass property value.",
    },
    NativeParameterSpec {
        name: "object",
        kind: "CDspObj handle",
        description: "Target display object.",
    },
    NativeParameterSpec {
        name: "parameter",
        kind: "i32 property number",
        description: "Passed to the target vtable slot +96 getter.",
    },
];
const GRAPH91_GET_OBJECT_POSITION_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "output",
        kind: "writable BP pointer to two i32",
        description: "Receives the resolved composite X/Y coordinates.",
    },
    NativeParameterSpec {
        name: "object",
        kind: "CDspObj handle",
        description: "Target display object.",
    },
];
const GRAPH91_ATTACH_OBJECT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "master",
        kind: "CDspObj handle",
        description: "Parent/master object.",
    },
    NativeParameterSpec {
        name: "slave",
        kind: "unowned CDspObj handle",
        description: "Child object attached to the master.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "i32",
        description: "Slave-local X offset.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32",
        description: "Slave-local Y offset.",
    },
];
const GRAPH91_DETACH_OBJECT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "master",
        kind: "CDspObj handle",
        description: "Current parent/master object.",
    },
    NativeParameterSpec {
        name: "slave",
        kind: "CDspObj handle",
        description: "Dependent child to detach.",
    },
];
const GRAPH91_MULTILAYER_INITIALIZE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "destination_x", kind: "signed16.16", description: "Layer0 destination anchor X." },
    NativeParameterSpec { name: "destination_y", kind: "signed16.16", description: "Layer0 destination anchor Y." },
    NativeParameterSpec { name: "bitmap", kind: "bitmap handle", description: "Live source with captured generation." },
    NativeParameterSpec { name: "source_x", kind: "signed16.16", description: "Source anchor X." },
    NativeParameterSpec { name: "source_y", kind: "signed16.16", description: "Source anchor Y." },
    NativeParameterSpec { name: "angle", kind: "16.16 degrees", description: "Layer0 rotation." },
    NativeParameterSpec { name: "scale_x", kind: "nonzero16.16", description: "Horizontal scale." },
    NativeParameterSpec { name: "scale_y", kind: "nonzero16.16", description: "Vertical scale." },
    NativeParameterSpec { name: "bilinear", kind: "BOOL", description: "Sampling flag, not transparency." },
];
const GRAPH91_MULTILAYER_INDEX_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "layer",
    kind: "0..=7",
    description: "Index into the eight CDspObjBackML layer records.",
}];
const GRAPH91_MULTILAYER_ENABLED_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "layer",
        kind: "0..=7",
        description: "Layer index.",
    },
    NativeParameterSpec {
        name: "enabled",
        kind: "i32 boolean",
        description: "Layer draw gate.",
    },
];
const GRAPH91_MULTILAYER_PAIR_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "layer",
        kind: "0..=7",
        description: "Layer index.",
    },
    NativeParameterSpec {
        name: "value_a",
        kind: "i32",
        description: "First selector-specific layer value.",
    },
    NativeParameterSpec {
        name: "value_b",
        kind: "i32",
        description: "Second selector-specific layer value.",
    },
];
const GRAPH91_MULTILAYER_SCALAR_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "layer",
        kind: "0..=7",
        description: "Layer index.",
    },
    NativeParameterSpec {
        name: "value",
        kind: "i32",
        description: "Selector-specific layer field.",
    },
];
const GRAPH91_MULTILAYER_BITMAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "layer",
        kind: "0..=7",
        description: "Layer index.",
    },
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap handle or -1",
        description: "Bitmap binding; -1 clears the record.",
    },
    NativeParameterSpec {
        name: "source_x",
        kind: "i32",
        description: "Source X coordinate.",
    },
    NativeParameterSpec {
        name: "source_y",
        kind: "i32",
        description: "Source Y coordinate.",
    },
];
const GRAPH91_MULTILAYER_TRANSFORM_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "layer", kind: "u32 <8", description: "Private layer index." },
    NativeParameterSpec { name: "angle", kind: "16.16 degrees", description: "Base rotation." },
    NativeParameterSpec { name: "scale_x", kind: "nonzero16.16", description: "Base horizontal scale." },
    NativeParameterSpec { name: "scale_y", kind: "nonzero16.16", description: "Base vertical scale." },
    NativeParameterSpec { name: "bilinear", kind: "BOOL", description: "Sampling selector; strength is configured separately by91:45." },
];
const GRAPH91_MULTILAYER_TRANSFORM_VELOCITY_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "layer", kind: "u32 <8", description: "Private layer index." },
    NativeParameterSpec { name: "angle_delta", kind: "16.16 degrees", description: "Coefficient applied through rotation curve to fixed8.24 progress." },
    NativeParameterSpec { name: "scale_x_delta", kind: "16.16", description: "Reciprocal interpolation target offset." },
    NativeParameterSpec { name: "scale_y_delta", kind: "16.16", description: "Reciprocal interpolation target offset." },
];

const GRAPH91_SPRITE_RELATION_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "sprite",
        kind: "0x80000000 tagged Sprite handle",
        description: "Primary CDspObjSprite whose target-native relation is changed.",
    },
    NativeParameterSpec {
        name: "linked_sprite",
        kind: "Sprite handle or 0",
        description: "Optional linked Sprite; zero clears the relation.",
    },
];
const GRAPH91_EFFECTOR_HANDLE_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "effector",
    kind: "0x91000000 tagged handle",
    description: "One of eight CDspObjEffector slots.",
}];
const GRAPH91_EFFECTOR_ENABLED_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "effector",
        kind: "0x91000000 tagged handle",
        description: "Target CDspObjEffector.",
    },
    NativeParameterSpec {
        name: "enabled",
        kind: "i32 boolean",
        description: "Native object enable gate.",
    },
];
const GRAPH91_DUAL_VECTOR_EFFECTOR_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "effector", kind: "effector handle", description: "CDspObjEffector." },
    NativeParameterSpec { name: "map1", kind: "format4 bitmap", description: "Must cover screen." },
    NativeParameterSpec { name: "map2", kind: "format4 bitmap or -1", description: "Must cover screen independently; dimensions may differ from first." },
    NativeParameterSpec { name: "strength", kind: "u32 <=256", description: "Native effect alpha, not output layer opacity." },
    NativeParameterSpec { name: "bilinear", kind: "BOOL", description: "Displacement sampling selector." },
    NativeParameterSpec { name: "priority", kind: "u32 <65536", description: "Display-chain priority." },
];
const GRAPH91_GRADIENT_EFFECTOR_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "effector", kind: "effector handle", description: "CDspObjEffector." },
    NativeParameterSpec { name: "edge_mode", kind: "enum0..1", description: "Horizontal blur black padding or edge clamp." },
    NativeParameterSpec { name: "radius", kind: "u32 <=256", description: "Native blur width stored as alpha." },
    NativeParameterSpec { name: "priority", kind: "u32 <65536", description: "Display-chain priority." },
];
const GRAPH91_RIPPLE_EFFECTOR_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "effector", kind: "effector handle", description: "CDspObjEffector." },
    NativeParameterSpec { name: "map", kind: "format6 bitmap", description: "Exactly screen extent." },
    NativeParameterSpec { name: "sample_count", kind: "nonzero u32", description: "Four copied wave samples per record." },
    NativeParameterSpec { name: "wave_slot", kind: "enum0..7", description: "Wave table slot, not BP pointer." },
    NativeParameterSpec { name: "amplitude", kind: "u32 <=256", description: "Wave amplitude multiplier." },
    NativeParameterSpec { name: "priority", kind: "u32 <65536", description: "Display-chain priority." },
];
const GRAPH91_TRANSFORM_EFFECTOR_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "effector", kind: "effector handle", description: "CDspObjEffector." },
    NativeParameterSpec { name: "source_x", kind: "signed16.16", description: "Source anchor X." },
    NativeParameterSpec { name: "source_y", kind: "signed16.16", description: "Source anchor Y." },
    NativeParameterSpec { name: "angle", kind: "16.16 degrees", description: "Inverse affine rotation." },
    NativeParameterSpec { name: "scale_x", kind: "nonzero unsigned16.16", description: "Horizontal scale at strength256." },
    NativeParameterSpec { name: "scale_y", kind: "nonzero unsigned16.16", description: "Vertical scale at strength256." },
    NativeParameterSpec { name: "bilinear", kind: "BOOL", description: "Four-bit bilinear sampling flag." },
    NativeParameterSpec { name: "strength", kind: "u32 <=256", description: "Native effect alpha, not output layer opacity." },
    NativeParameterSpec { name: "priority", kind: "u32 <65536", description: "Display-chain priority." },
];
const GRAPH91_SURFACE_EFFECTOR_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "effector", kind: "effector handle", description: "CDspObjEffector." },
    NativeParameterSpec { name: "retention", kind: "u32 <=256", description: "Weight of retained previous result; configuration clears it to black." },
    NativeParameterSpec { name: "priority", kind: "u32 <65536", description: "Display-chain priority." },
];
const GRAPH91_CREATE_LANDSCAPE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "cell_width",
        kind: "even i32 or 0",
        description: "Hex/staggered cell width; zero or odd selects 64.",
    },
    NativeParameterSpec {
        name: "row_step",
        kind: "i32 or 0",
        description: "Vertical row step; zero selects 16.",
    },
    NativeParameterSpec {
        name: "column_step",
        kind: "i32 or 0",
        description: "Target column-step field; zero selects 16.",
    },
    NativeParameterSpec {
        name: "baseline",
        kind: "i32 or 0",
        description: "Priority baseline; zero selects 35 * row_step.",
    },
    NativeParameterSpec {
        name: "priority_row_factor",
        kind: "i32",
        description: "Per-row priority contribution.",
    },
    NativeParameterSpec {
        name: "priority_frame_factor",
        kind: "i32",
        description: "Per-column/frame priority contribution.",
    },
];
const GRAPH91_LANDSCAPE_HANDLE_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "landscape",
    kind: "0xA1000000 tagged handle",
    description: "One of four CDspObjLandscape slots.",
}];
const GRAPH91_LANDSCAPE_HIT_TEST_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "cell_out",
        kind: "writable BP pointer to two i32",
        description: "Receives line and column when hit.",
    },
    NativeParameterSpec {
        name: "landscape",
        kind: "landscape handle",
        description: "Target CDspObjLandscape.",
    },
    NativeParameterSpec {
        name: "alpha_test_mode",
        kind: "i32",
        description: "Selects the target alternate image-mask test when equal to one.",
    },
];
const GRAPH91_LANDSCAPE_ENABLED_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "landscape",
        kind: "landscape handle",
        description: "Target CDspObjLandscape.",
    },
    NativeParameterSpec {
        name: "enabled",
        kind: "i32 boolean",
        description: "Native object enable gate.",
    },
];
const GRAPH91_LANDSCAPE_OBJECT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "landscape",
        kind: "landscape handle",
        description: "Target CDspObjLandscape.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "i32",
        description: "Object X position.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32",
        description: "Object Y position.",
    },
    NativeParameterSpec {
        name: "blend_mode",
        kind: "i32",
        description: "Native blend selector.",
    },
    NativeParameterSpec {
        name: "alpha",
        kind: "i32",
        description: "Base composition/transparency value.",
    },
    NativeParameterSpec {
        name: "priority",
        kind: "i32",
        description: "Display-object priority.",
    },
];
const GRAPH91_LANDSCAPE_SILHOUETTE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "landscape",
        kind: "landscape handle",
        description: "Target CDspObjLandscape.",
    },
    NativeParameterSpec {
        name: "line",
        kind: "map line",
        description: "Validated against configured map height.",
    },
    NativeParameterSpec {
        name: "column",
        kind: "map column",
        description: "Validated against configured map width.",
    },
    NativeParameterSpec {
        name: "silhouette_type",
        kind: "0..=2",
        description: "Target silhouette mode.",
    },
    NativeParameterSpec {
        name: "silhouette_level",
        kind: "0..=256",
        description: "Target silhouette level.",
    },
    NativeParameterSpec {
        name: "silhouette_value",
        kind: "i32",
        description: "Third silhouette field in the 68-byte cell record.",
    },
];
const GRAPH91_LANDSCAPE_PARTS_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "landscape",
        kind: "landscape handle",
        description: "Target CDspObjLandscape.",
    },
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap handle",
        description: "Source sheet for parts and pillars/columns.",
    },
    NativeParameterSpec {
        name: "part_count",
        kind: "nonnegative i32",
        description: "Number of five-DWORD part descriptors.",
    },
    NativeParameterSpec {
        name: "parts",
        kind: "readable BP pointer",
        description: "part_count * 5 DWORD descriptors.",
    },
    NativeParameterSpec {
        name: "part_spacing",
        kind: "i32",
        description: "Target part-layout spacing field.",
    },
    NativeParameterSpec {
        name: "column_count",
        kind: "nonnegative i32",
        description: "Number of 34-DWORD pillar/column descriptors.",
    },
    NativeParameterSpec {
        name: "columns",
        kind: "readable BP pointer",
        description: "column_count * 34 DWORD descriptors.",
    },
];
const GRAPH91_LANDSCAPE_MAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "landscape",
        kind: "landscape handle",
        description: "Target CDspObjLandscape.",
    },
    NativeParameterSpec {
        name: "width",
        kind: "1..=256",
        description: "Map width in cells.",
    },
    NativeParameterSpec {
        name: "height",
        kind: "1..=256",
        description: "Map height in cells.",
    },
    NativeParameterSpec {
        name: "map",
        kind: "readable BP pointer",
        description: "width * height i32 pillar/column indices.",
    },
];
const GRAPH91_LANDSCAPE_GUIDES_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "landscape",
        kind: "landscape handle",
        description: "Target CDspObjLandscape.",
    },
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap handle",
        description: "Guide-image sheet.",
    },
    NativeParameterSpec {
        name: "guide_count",
        kind: "nonnegative i32",
        description: "Number of guide descriptors.",
    },
    NativeParameterSpec {
        name: "guides",
        kind: "readable BP pointer",
        description: "guide_count * 5 DWORD descriptors.",
    },
];
const GRAPH91_LANDSCAPE_CELL_GUIDES_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "landscape",
        kind: "landscape handle",
        description: "Target CDspObjLandscape.",
    },
    NativeParameterSpec {
        name: "point_count",
        kind: "nonnegative i32",
        description: "Number of column/line pairs.",
    },
    NativeParameterSpec {
        name: "points",
        kind: "readable BP pointer",
        description: "point_count * 2 DWORD pairs.",
    },
    NativeParameterSpec {
        name: "layer",
        kind: "0..=3",
        description: "Cell guide layer.",
    },
    NativeParameterSpec {
        name: "guide",
        kind: "guide index or -1",
        description: "Guide record assigned to each cell.",
    },
    NativeParameterSpec {
        name: "value",
        kind: "i32",
        description: "Guide-associated cell value.",
    },
];
const GRAPH91_LANDSCAPE_PART_COPY_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "landscape",
        kind: "landscape handle",
        description: "Target CDspObjLandscape.",
    },
    NativeParameterSpec {
        name: "destination_part",
        kind: "part index",
        description: "Part record receiving the source image.",
    },
    NativeParameterSpec {
        name: "source_part",
        kind: "part index",
        description: "Dimension-compatible source part.",
    },
];
const GRAPH91_LANDSCAPE_CELL_COLUMN_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "landscape",
        kind: "landscape handle",
        description: "Target CDspObjLandscape.",
    },
    NativeParameterSpec {
        name: "line",
        kind: "map line",
        description: "Cell line.",
    },
    NativeParameterSpec {
        name: "column",
        kind: "map column",
        description: "Cell column.",
    },
    NativeParameterSpec {
        name: "column_index",
        kind: "pillar/column index or -1",
        description: "New cell pillar/column record.",
    },
];
const GRAPH91_LANDSCAPE_CELL_VALUE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "value_out",
        kind: "writable BP pointer to i32",
        description: "Receives the zero-extended target cell WORD.",
    },
    NativeParameterSpec {
        name: "landscape",
        kind: "landscape handle",
        description: "Target CDspObjLandscape.",
    },
    NativeParameterSpec {
        name: "line",
        kind: "map line",
        description: "Cell line.",
    },
    NativeParameterSpec {
        name: "column",
        kind: "map column",
        description: "Cell column.",
    },
];
const GRAPH91_LANDSCAPE_CELL_IMAGE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "existing bitmap handle",
        description: "Receives the selected cell-part image.",
    },
    NativeParameterSpec {
        name: "landscape",
        kind: "landscape handle",
        description: "Target CDspObjLandscape.",
    },
    NativeParameterSpec {
        name: "line",
        kind: "map line",
        description: "Cell line.",
    },
    NativeParameterSpec {
        name: "column",
        kind: "map column",
        description: "Cell column.",
    },
];

const GRAPH91_WINDOW_FONT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "window",
        kind: "CDspObjWindow handle",
        description: "Window whose native text renderer is configured.",
    },
    NativeParameterSpec {
        name: "font_name_id",
        kind: "font name/string id",
        description: "Resolved by target sub_468BB0.",
    },
    NativeParameterSpec {
        name: "font_size",
        kind: "i32 4..=200",
        description: "Stored at CDspObjWindow+0x358.",
    },
    NativeParameterSpec {
        name: "scale_percent",
        kind: "i32 25..=200",
        description: "Scaled dimension is font_size * scale_percent / 100 and is stored at +0x35C.",
    },
    NativeParameterSpec {
        name: "font_style",
        kind: "i32",
        description: "Native font resource style field.",
    },
    NativeParameterSpec {
        name: "layout_option",
        kind: "i32",
        description: "Stored at CDspObjWindow+0x354.",
    },
    NativeParameterSpec {
        name: "render_option",
        kind: "i32",
        description: "Stored at CDspObjWindow+0x364.",
    },
];

const GRAPH91_WINDOW_LINE_SPACING_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "window",
        kind: "CDspObjWindow handle",
        description: "Target window.",
    },
    NativeParameterSpec {
        name: "line_spacing_percent",
        kind: "i32 0..=800",
        description: "Stored at CDspObjWindow+0x360.",
    },
];

const GRAPH91_WINDOW_LAYOUT_MODE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "window",
        kind: "CDspObjWindow handle",
        description: "Target window.",
    },
    NativeParameterSpec {
        name: "text_layout_mode",
        kind: "i32 enum 0..=2",
        description: "Stored at CDspObjWindow+0x374.",
    },
];

const GRAPH91_WINDOW_CURSOR_SET_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "window",
        kind: "CDspObjWindow handle",
        description: "Target window.",
    },
    NativeParameterSpec {
        name: "cursor_x",
        kind: "i32",
        description: "Stored at CDspObjWindow+0x368.",
    },
    NativeParameterSpec {
        name: "cursor_y",
        kind: "i32",
        description: "Stored at CDspObjWindow+0x36C.",
    },
];

const GRAPH91_WINDOW_HANDLE_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "window",
    kind: "CDspObjWindow handle",
    description: "Target window.",
}];

const GRAPH91_MESSAGE_EX_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "argument_1",
        kind: "i32/string native field",
        description: "Target CProcDspMsgEx constructor argument in BP source order.",
    },
    NativeParameterSpec {
        name: "argument_2",
        kind: "i32/string native field",
        description: "Target CProcDspMsgEx constructor argument in BP source order.",
    },
    NativeParameterSpec {
        name: "argument_3",
        kind: "i32/string native field",
        description: "Target CProcDspMsgEx constructor argument in BP source order.",
    },
    NativeParameterSpec {
        name: "argument_4",
        kind: "i32/string native field",
        description: "Target CProcDspMsgEx constructor argument in BP source order.",
    },
    NativeParameterSpec {
        name: "argument_5",
        kind: "i32/string native field",
        description: "Target CProcDspMsgEx constructor argument in BP source order.",
    },
    NativeParameterSpec {
        name: "argument_6",
        kind: "i32/string native field",
        description: "Target CProcDspMsgEx constructor argument in BP source order.",
    },
    NativeParameterSpec {
        name: "argument_7",
        kind: "i32/string native field",
        description: "Target CProcDspMsgEx constructor argument in BP source order.",
    },
    NativeParameterSpec {
        name: "argument_8",
        kind: "i32/string native field",
        description: "Target CProcDspMsgEx constructor argument in BP source order.",
    },
    NativeParameterSpec {
        name: "argument_9",
        kind: "i32/string native field",
        description: "Target CProcDspMsgEx constructor argument in BP source order.",
    },
    NativeParameterSpec {
        name: "argument_10",
        kind: "i32/string native field",
        description: "Target CProcDspMsgEx constructor argument in BP source order.",
    },
];

const GRAPH91_MESSAGE_EX_OPTION_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "argument_1",
        kind: "i32/string native field",
        description: "Target CProcDspMsgEx constructor argument in BP source order; argument 11 is the extended option.",
    },
    NativeParameterSpec {
        name: "argument_2",
        kind: "i32/string native field",
        description: "Target CProcDspMsgEx constructor argument in BP source order; argument 11 is the extended option.",
    },
    NativeParameterSpec {
        name: "argument_3",
        kind: "i32/string native field",
        description: "Target CProcDspMsgEx constructor argument in BP source order; argument 11 is the extended option.",
    },
    NativeParameterSpec {
        name: "argument_4",
        kind: "i32/string native field",
        description: "Target CProcDspMsgEx constructor argument in BP source order; argument 11 is the extended option.",
    },
    NativeParameterSpec {
        name: "argument_5",
        kind: "i32/string native field",
        description: "Target CProcDspMsgEx constructor argument in BP source order; argument 11 is the extended option.",
    },
    NativeParameterSpec {
        name: "argument_6",
        kind: "i32/string native field",
        description: "Target CProcDspMsgEx constructor argument in BP source order; argument 11 is the extended option.",
    },
    NativeParameterSpec {
        name: "argument_7",
        kind: "i32/string native field",
        description: "Target CProcDspMsgEx constructor argument in BP source order; argument 11 is the extended option.",
    },
    NativeParameterSpec {
        name: "argument_8",
        kind: "i32/string native field",
        description: "Target CProcDspMsgEx constructor argument in BP source order; argument 11 is the extended option.",
    },
    NativeParameterSpec {
        name: "argument_9",
        kind: "i32/string native field",
        description: "Target CProcDspMsgEx constructor argument in BP source order; argument 11 is the extended option.",
    },
    NativeParameterSpec {
        name: "argument_10",
        kind: "i32/string native field",
        description: "Target CProcDspMsgEx constructor argument in BP source order; argument 11 is the extended option.",
    },
    NativeParameterSpec {
        name: "argument_11",
        kind: "i32/string native field",
        description: "Target CProcDspMsgEx constructor argument in BP source order; argument 11 is the extended option.",
    },
];

const GRAPH91_RENDER_TEXT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "argument_1",
        kind: "i32/string renderer field",
        description: "Immediate window-text renderer argument in BP source order.",
    },
    NativeParameterSpec {
        name: "argument_2",
        kind: "i32/string renderer field",
        description: "Immediate window-text renderer argument in BP source order.",
    },
    NativeParameterSpec {
        name: "argument_3",
        kind: "i32/string renderer field",
        description: "Immediate window-text renderer argument in BP source order.",
    },
    NativeParameterSpec {
        name: "argument_4",
        kind: "i32/string renderer field",
        description: "Immediate window-text renderer argument in BP source order.",
    },
    NativeParameterSpec {
        name: "argument_5",
        kind: "i32/string renderer field",
        description: "Immediate window-text renderer argument in BP source order.",
    },
];

const GRAPH91_RENDER_TEXT_STYLE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "argument_1",
        kind: "i32/string renderer field",
        description: "Immediate renderer argument; the additional flag selects default versus zeroed style state.",
    },
    NativeParameterSpec {
        name: "argument_2",
        kind: "i32/string renderer field",
        description: "Immediate renderer argument; the additional flag selects default versus zeroed style state.",
    },
    NativeParameterSpec {
        name: "argument_3",
        kind: "i32/string renderer field",
        description: "Immediate renderer argument; the additional flag selects default versus zeroed style state.",
    },
    NativeParameterSpec {
        name: "argument_4",
        kind: "i32/string renderer field",
        description: "Immediate renderer argument; the additional flag selects default versus zeroed style state.",
    },
    NativeParameterSpec {
        name: "argument_5",
        kind: "i32/string renderer field",
        description: "Immediate renderer argument; the additional flag selects default versus zeroed style state.",
    },
    NativeParameterSpec {
        name: "argument_6",
        kind: "i32/string renderer field",
        description: "Immediate renderer argument; the additional flag selects default versus zeroed style state.",
    },
];

const GRAPH91_TEXT_SUBSTITUTION_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "source",
        kind: "nullable string",
        description: "Null clears all substitutions; a nonnull source identifies the mapping.",
    },
    NativeParameterSpec {
        name: "replacement",
        kind: "nullable string",
        description: "Nonnull adds/updates; null removes the named source mapping.",
    },
];

const GRAPH91_TEXT_SUBSTITUTION_COUNT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "ignored",
        kind: "converted native argument",
        description: "Accepted by the wrapper but not used by the target core.",
    },
    NativeParameterSpec {
        name: "source",
        kind: "string",
        description: "Text scanned against the substitution dictionary.",
    },
];

const GRAPH91_TEXT_SUBSTITUTION_RECORDS_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "records",
        kind: "string",
        description: "Newline-delimited base\\reading records registered atomically until malformed input.",
    },
];

const GRAPH91_TEXT_STYLE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "font_name_id",
        kind: "font name/string id",
        description: "Resolved by target sub_468BB0.",
    },
    NativeParameterSpec {
        name: "style_value_0",
        kind: "i32",
        description: "Persistent style field.",
    },
    NativeParameterSpec {
        name: "style_value_1",
        kind: "i32",
        description: "Persistent style field.",
    },
    NativeParameterSpec {
        name: "style_value_2",
        kind: "i32",
        description: "Persistent style field.",
    },
    NativeParameterSpec {
        name: "style_value_3",
        kind: "i32",
        description: "Persistent style field.",
    },
];

const GRAPH91_TEXT_LAYOUT_GLOBAL_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "layout_advance",
        kind: "i32",
        description: "Stored in dword_507638.",
    },
    NativeParameterSpec {
        name: "scale_denominator",
        kind: "i32 (zero becomes one)",
        description: "Stored in dword_50763C.",
    },
    NativeParameterSpec {
        name: "character_spacing",
        kind: "i32",
        description: "Stored in dword_565BB0 and added by sub_436FF0.",
    },
    NativeParameterSpec {
        name: "font_percent",
        kind: "i32 25..=100",
        description: "Stored in dword_507640.",
    },
    NativeParameterSpec {
        name: "boundary_offset",
        kind: "nonnegative i32",
        description: "Stored in dword_565BDC and used by 91:8E.",
    },
    NativeParameterSpec {
        name: "mode_flag",
        kind: "i32",
        description: "Stored in dword_565CF0.",
    },
];

const GRAPH91_TEXT_SCALE_DIVISOR_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "divisor",
    kind: "nonzero i32",
    description: "Stores (divisor + 0xFFFF) / divisor as a 16.16 reciprocal.",
}];

const GRAPH91_TEXT_GLOBAL_PROPERTY_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "property",
        kind: "0 or 0x80000000..2",
        description: "Selects one of four target global fields.",
    },
    NativeParameterSpec {
        name: "value",
        kind: "i32",
        description: "Property 0x80000001 requires a nonnegative value.",
    },
];

const GRAPH91_TEXT_MEASURE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "width_out",
        kind: "writable BP pointer",
        description: "Receives one measured DWORD.",
    },
    NativeParameterSpec {
        name: "text",
        kind: "string",
        description: "Text to measure.",
    },
    NativeParameterSpec {
        name: "max_width",
        kind: "i32",
        description: "Native metric constraint.",
    },
    NativeParameterSpec {
        name: "font_size",
        kind: "i32",
        description: "Native font/size validation argument.",
    },
    NativeParameterSpec {
        name: "scale",
        kind: "i32",
        description: "Additional text metric argument.",
    },
    NativeParameterSpec {
        name: "style",
        kind: "i32",
        description: "Style/control argument.",
    },
    NativeParameterSpec {
        name: "flags",
        kind: "i32",
        description: "Final renderer flags.",
    },
];

const GRAPH91_DRAW_TEXT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "argument_1",
        kind: "i32/string/pointer renderer field",
        description: "Target sub_403B10/sub_434BA0 argument in BP source order.",
    },
    NativeParameterSpec {
        name: "argument_2",
        kind: "i32/string/pointer renderer field",
        description: "Target sub_403B10/sub_434BA0 argument in BP source order.",
    },
    NativeParameterSpec {
        name: "argument_3",
        kind: "i32/string/pointer renderer field",
        description: "Target sub_403B10/sub_434BA0 argument in BP source order.",
    },
    NativeParameterSpec {
        name: "argument_4",
        kind: "i32/string/pointer renderer field",
        description: "Target sub_403B10/sub_434BA0 argument in BP source order.",
    },
    NativeParameterSpec {
        name: "argument_5",
        kind: "i32/string/pointer renderer field",
        description: "Target sub_403B10/sub_434BA0 argument in BP source order.",
    },
    NativeParameterSpec {
        name: "argument_6",
        kind: "i32/string/pointer renderer field",
        description: "Target sub_403B10/sub_434BA0 argument in BP source order.",
    },
    NativeParameterSpec {
        name: "argument_7",
        kind: "i32/string/pointer renderer field",
        description: "Target sub_403B10/sub_434BA0 argument in BP source order.",
    },
    NativeParameterSpec {
        name: "argument_8",
        kind: "i32/string/pointer renderer field",
        description: "Target sub_403B10/sub_434BA0 argument in BP source order.",
    },
    NativeParameterSpec {
        name: "argument_9",
        kind: "i32/string/pointer renderer field",
        description: "Target sub_403B10/sub_434BA0 argument in BP source order.",
    },
    NativeParameterSpec {
        name: "argument_10",
        kind: "i32/string/pointer renderer field",
        description: "Target sub_403B10/sub_434BA0 argument in BP source order.",
    },
    NativeParameterSpec {
        name: "argument_11",
        kind: "i32/string/pointer renderer field",
        description: "Target sub_403B10/sub_434BA0 argument in BP source order.",
    },
    NativeParameterSpec {
        name: "argument_12",
        kind: "i32/string/pointer renderer field",
        description: "Target sub_403B10/sub_434BA0 argument in BP source order.",
    },
    NativeParameterSpec {
        name: "argument_13",
        kind: "i32/string/pointer renderer field",
        description: "Target sub_403B10/sub_434BA0 argument in BP source order.",
    },
    NativeParameterSpec {
        name: "argument_14",
        kind: "i32/string/pointer renderer field",
        description: "Target sub_403B10/sub_434BA0 argument in BP source order.",
    },
];

const GRAPH91_DRAW_TEXT_STYLE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "argument_1",
        kind: "i32/string/pointer renderer field",
        description: "Extended renderer argument; the additional flag selects default versus zeroed style.",
    },
    NativeParameterSpec {
        name: "argument_2",
        kind: "i32/string/pointer renderer field",
        description: "Extended renderer argument; the additional flag selects default versus zeroed style.",
    },
    NativeParameterSpec {
        name: "argument_3",
        kind: "i32/string/pointer renderer field",
        description: "Extended renderer argument; the additional flag selects default versus zeroed style.",
    },
    NativeParameterSpec {
        name: "argument_4",
        kind: "i32/string/pointer renderer field",
        description: "Extended renderer argument; the additional flag selects default versus zeroed style.",
    },
    NativeParameterSpec {
        name: "argument_5",
        kind: "i32/string/pointer renderer field",
        description: "Extended renderer argument; the additional flag selects default versus zeroed style.",
    },
    NativeParameterSpec {
        name: "argument_6",
        kind: "i32/string/pointer renderer field",
        description: "Extended renderer argument; the additional flag selects default versus zeroed style.",
    },
    NativeParameterSpec {
        name: "argument_7",
        kind: "i32/string/pointer renderer field",
        description: "Extended renderer argument; the additional flag selects default versus zeroed style.",
    },
    NativeParameterSpec {
        name: "argument_8",
        kind: "i32/string/pointer renderer field",
        description: "Extended renderer argument; the additional flag selects default versus zeroed style.",
    },
    NativeParameterSpec {
        name: "argument_9",
        kind: "i32/string/pointer renderer field",
        description: "Extended renderer argument; the additional flag selects default versus zeroed style.",
    },
    NativeParameterSpec {
        name: "argument_10",
        kind: "i32/string/pointer renderer field",
        description: "Extended renderer argument; the additional flag selects default versus zeroed style.",
    },
    NativeParameterSpec {
        name: "argument_11",
        kind: "i32/string/pointer renderer field",
        description: "Extended renderer argument; the additional flag selects default versus zeroed style.",
    },
    NativeParameterSpec {
        name: "argument_12",
        kind: "i32/string/pointer renderer field",
        description: "Extended renderer argument; the additional flag selects default versus zeroed style.",
    },
    NativeParameterSpec {
        name: "argument_13",
        kind: "i32/string/pointer renderer field",
        description: "Extended renderer argument; the additional flag selects default versus zeroed style.",
    },
    NativeParameterSpec {
        name: "argument_14",
        kind: "i32/string/pointer renderer field",
        description: "Extended renderer argument; the additional flag selects default versus zeroed style.",
    },
    NativeParameterSpec {
        name: "argument_15",
        kind: "i32/string/pointer renderer field",
        description: "Extended renderer argument; the additional flag selects default versus zeroed style.",
    },
];

const GRAPH91_TEXT_LABEL_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "writable BP pointer",
        description: "Receives consecutive zero-filled 128-byte records.",
    },
    NativeParameterSpec {
        name: "source",
        kind: "string",
        description: "Markup source containing <l>...</l> labels.",
    },
];

const GRAPH91_STRIP_MARKUP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "writable BP pointer",
        description: "Receives the stripped C string.",
    },
    NativeParameterSpec {
        name: "source",
        kind: "string",
        description: "Input markup string.",
    },
];

const GRAPH91_EXTENDED_ICON_CREATE_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "window",
    kind: "0xB0000000 tagged Window handle",
    description: "Window that owns the extended icon-input processor.",
}];
const GRAPH91_EXTENDED_ICON_CONFIGURE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "input_object",
        kind: "extended icon-input handle",
        description: "Object created by Graph91:B8.",
    },
    NativeParameterSpec {
        name: "descriptor",
        kind: "readable BP pointer",
        description: "Extended 40/64/196-byte root/group/item tree. Root +0x14 suppresses physical action bits, +0x18 selects action-map mode 0..7, and +0x20 nonzero disables pointer processing. Group +0x0C current item, +0x14 selection enable, +0x18 pointer-selection enable, +0x1C held mouse-left action reinjection, +0x20 exclusion key, +0x3C flags (bit 0x02 selects release-time activation). Item +0x20 normal, +0x24 hover, +0x28 selected, +0x2C hover+selected, +0x30 auxiliary, +0xC0 flags (bit 0x20 also selects release-time activation). When either timing bit is set, sub_44C6F0 returns false on MouseDown; sub_448690 stores the hit in DCIPIcon+0x90 and activates on MouseRelease if the pointer remains over the same item. These bits do not disable the item.",
    },
];
const GRAPH91_EXTENDED_ICON_ITEM_STATE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "input_object",
        kind: "extended icon-input handle",
        description: "Object created by Graph91:B8.",
    },
    NativeParameterSpec {
        name: "group_index",
        kind: "i32",
        description: "Configured descriptor group index.",
    },
    NativeParameterSpec {
        name: "item_index",
        kind: "i32",
        description: "Configured item/region index inside the group.",
    },
    NativeParameterSpec {
        name: "state",
        kind: "i32",
        description: "Visual/frame state forwarded to the item's child display object.",
    },
];
const GRAPH91_KEY_ASSIGNMENT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "assignment_id",
        kind: "4..=7",
        description: "Selects one of four process-global key-assignment records.",
    },
    NativeParameterSpec {
        name: "table",
        kind: "readable BP pointer to 24 i32",
        description: "Exactly 24 DWORDs copied into the selected record.",
    },
];
const GRAPH91_MOVIE_OPEN_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap slot 0..0x3FFF",
        description: "Destination movie slot and decoded frame bitmap.",
    },
    NativeParameterSpec {
        name: "path",
        kind: "resource/file string",
        description: "DirectShow movie source path or packed resource name.",
    },
    NativeParameterSpec {
        name: "loop_flag",
        kind: "i32 boolean",
        description: "Nonzero enables looping.",
    },
    NativeParameterSpec {
        name: "volume",
        kind: "0..=128",
        description: "Initial movie volume.",
    },
];
const GRAPH91_MOVIE_START_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "duration_out",
        kind: "writable BP pointer to i32",
        description: "Receives media duration in milliseconds after preparation.",
    },
    NativeParameterSpec {
        name: "bitmap",
        kind: "movie bitmap slot",
        description: "Movie opened by Graph91:F0.",
    },
];
const GRAPH91_MOVIE_HANDLE_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "bitmap",
    kind: "movie bitmap slot",
    description: "Target DirectShow/portable movie slot.",
}];
const GRAPH91_MOVIE_PAUSE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "bitmap",
        kind: "movie bitmap slot",
        description: "Prepared movie slot.",
    },
    NativeParameterSpec {
        name: "paused",
        kind: "i32 boolean",
        description: "Zero resumes; nonzero pauses.",
    },
];
const GRAPH91_FLASH_CREATE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap slot 0..0x3FFF",
        description: "Destination bitmap receiving Flash-rendered pixels.",
    },
    NativeParameterSpec {
        name: "width",
        kind: "positive i32",
        description: "Hidden ActiveX host width.",
    },
    NativeParameterSpec {
        name: "height",
        kind: "positive i32",
        description: "Hidden ActiveX host height.",
    },
    NativeParameterSpec {
        name: "path",
        kind: "SWF path string",
        description: "Shockwave Flash movie assigned to the ActiveX control.",
    },
    NativeParameterSpec {
        name: "parameter",
        kind: "i32",
        description: "Recovered SCFlashControl field +0x1C; exact consumer meaning remains open.",
    },
];
const GRAPH91_MOVIE_POSITION_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "position_out",
        kind: "writable BP pointer to i32",
        description: "Receives current media playback position.",
    },
    NativeParameterSpec {
        name: "bitmap",
        kind: "movie bitmap slot",
        description: "DirectShow movie slot.",
    },
];

const GRAPH90_KNOB_HANDLE_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "knob",
    kind: "0xF0000000 tagged handle",
    description: "CDspObjKnob registry handle.",
}];
const GRAPH90_CREATE_KNOB_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "target",
    kind: "display-object handle",
    description: "Unowned, nonvirtual target controlled by the knob.",
}];
const GRAPH90_KNOB_ENABLED_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "knob",
        kind: "knob handle",
        description: "Target CDspObjKnob.",
    },
    NativeParameterSpec {
        name: "enabled",
        kind: "bool/i32",
        description: "Native draw/input enable gate.",
    },
];
const GRAPH90_KNOB_PAIR_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "knob",
        kind: "knob handle",
        description: "Target CDspObjKnob.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "i32",
        description: "Horizontal value.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32",
        description: "Vertical value.",
    },
];
const GRAPH90_KNOB_MODE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "knob",
        kind: "knob handle",
        description: "Target CDspObjKnob.",
    },
    NativeParameterSpec {
        name: "mode",
        kind: "i32",
        description: "Raw relative-offset mode stored at CDspObjKnob+0x140.",
    },
];
const GRAPH90_SWAP_KNOB_INPUT_MODE_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "mode",
    kind: "i32",
    description: "New process-global knob input mode; previous value is returned.",
}];
const GRAPH90_GROUP_HANDLE_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "group",
    kind: "0xF1000000 tagged handle",
    description: "CDspObjGroup registry handle.",
}];
const GRAPH90_GROUP_ENABLED_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "group",
        kind: "group handle",
        description: "Target CDspObjGroup.",
    },
    NativeParameterSpec {
        name: "enabled",
        kind: "bool/i32",
        description: "Recursive CDspObj draw-enable gate (+0x14), propagated to current group members.",
    },
];
const GRAPH90_CONFIGURE_GROUP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "group",
        kind: "group handle",
        description: "Target CDspObjGroup.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "i32",
        description: "Group X position.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32",
        description: "Group Y position.",
    },
    NativeParameterSpec {
        name: "alpha_parameter",
        kind: "i32 transparency parameter",
        description: "Passed to CDspObjGroup vtable+0x48 SetAlpha; propagated to current member objects by the native CDspObj setter.",
    },
];
const GRAPH90_ADD_GROUP_OBJECT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "group",
        kind: "group handle",
        description: "Destination CDspObjGroup.",
    },
    NativeParameterSpec {
        name: "object",
        kind: "display-object handle",
        description: "Unowned object to attach.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "i32",
        description: "Member-local X offset.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32",
        description: "Member-local Y offset.",
    },
];
const GRAPH90_REMOVE_GROUP_OBJECT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "group",
        kind: "group handle",
        description: "Source CDspObjGroup.",
    },
    NativeParameterSpec {
        name: "object",
        kind: "display-object handle",
        description: "Registered member to detach.",
    },
];

const GRAPH_RENDER_OBJECT_TO_BITMAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination_bitmap",
        kind: "bitmap handle",
        description: "First BP argument; receives the rendered graph/display object image.",
    },
    NativeParameterSpec {
        name: "source_window",
        kind: "window handle",
        description: "Second BP argument; target sub_440C80 renders this CDspObjWindow into destination_bitmap.",
    },
];
const GRAPH_BIND_BITMAP_TO_SURFACE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "window",
        kind: "window handle",
        description: "First BP argument; target CDspObjWindow whose background backing is updated.",
    },
    NativeParameterSpec {
        name: "decoration_error_context",
        kind: "i32 diagnostic context",
        description: "Second BP argument; not passed to the target core and only used in error diagnostics.",
    },
    NativeParameterSpec {
        name: "frame_error_context",
        kind: "i32 diagnostic context",
        description: "Third BP argument; not passed to the target core and only used in error diagnostics.",
    },
    NativeParameterSpec {
        name: "source_bitmap",
        kind: "bitmap handle",
        description: "Fourth BP argument; target sub_440780 maps this bitmap to destination_surface.",
    },
];
const GRAPH92_CONFIGURE_COMPACT_WAVE_TABLE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "slot",
        kind: "i32 0..=7",
        description: "Target process-global wave-table slot.",
    },
    NativeParameterSpec {
        name: "quarter_period",
        kind: "i32",
        description: "Quarter-cycle sample count; zero is normalized to one.",
    },
    NativeParameterSpec {
        name: "amplitude",
        kind: "i32",
        description: "Signed sine-wave amplitude converted to i16 samples.",
    },
    NativeParameterSpec {
        name: "block_height",
        kind: "i32",
        description: "Rows per block; zero is normalized to one.",
    },
    NativeParameterSpec {
        name: "block_count",
        kind: "i32",
        description: "Number of repeated blocks; zero is normalized to one.",
    },
];
const GRAPH92_CONFIGURE_WAVE_TABLE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "slot",
        kind: "i32 0..=7",
        description: "Target process-global wave-table slot.",
    },
    NativeParameterSpec {
        name: "quarter_period",
        kind: "i32",
        description: "Quarter-cycle sample count; zero is normalized to one.",
    },
    NativeParameterSpec {
        name: "amplitude",
        kind: "i32",
        description: "Full sine-wave amplitude.",
    },
    NativeParameterSpec {
        name: "lead_levels",
        kind: "nonnegative i32",
        description: "Number of amplitude-halving cycles before the full cycle.",
    },
    NativeParameterSpec {
        name: "trail_levels",
        kind: "nonnegative i32",
        description: "Number of amplitude-halving cycles after the full cycle.",
    },
    NativeParameterSpec {
        name: "block_height",
        kind: "i32",
        description: "Rows per block; all but the last row are zero-filled.",
    },
    NativeParameterSpec {
        name: "block_count",
        kind: "i32",
        description: "Number of repeated blocks.",
    },
];
const GRAPH92_RADIAL_VECTOR_MAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "bitmap",
        kind: "format-6 bitmap handle",
        description: "Destination vector/phase map.",
    },
    NativeParameterSpec {
        name: "mode",
        kind: "i32 enum 0..=1",
        description: "Selects normal or component-swapped radial vectors.",
    },
    NativeParameterSpec {
        name: "center_x",
        kind: "i32",
        description: "Radial center X.",
    },
    NativeParameterSpec {
        name: "center_y",
        kind: "i32",
        description: "Radial center Y.",
    },
    NativeParameterSpec {
        name: "period",
        kind: "i32",
        description: "Quarter phase period; nonpositive selects the target nonwrapping divisor.",
    },
];
const GRAPH92_AXIS_VECTOR_MAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "bitmap",
        kind: "format-6 bitmap handle",
        description: "Destination vector/phase map.",
    },
    NativeParameterSpec {
        name: "mode",
        kind: "i32 enum 0..=3",
        description: "Selects one of four fixed vector and phase-axis combinations.",
    },
];
const GRAPH92_BITMAP_AUXILIARY_PAIR_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap handle",
        description: "Descriptor whose auxiliary DWORD pair is updated.",
    },
    NativeParameterSpec {
        name: "reference_x",
        kind: "i32",
        description: "Auxiliary reference-point X stored at target bitmap-registry offset +0x28.",
    },
    NativeParameterSpec {
        name: "reference_y",
        kind: "i32",
        description: "Auxiliary reference-point Y stored at target bitmap-registry offset +0x2C.",
    },
];
const GRAPH92_REPLACE_BITMAP_COLOR_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "bitmap",
        kind: "format-1/2 bitmap handle",
        description: "Bitmap modified in place.",
    },
    NativeParameterSpec {
        name: "needle_bgra",
        kind: "packed native color",
        description: "Exact RGB or BGRA value selected according to source format and alpha.",
    },
    NativeParameterSpec {
        name: "replacement_bgra",
        kind: "packed native color",
        description: "Replacement native color.",
    },
];
const GRAPH92_PRELOAD_BITMAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "resource_name",
        kind: "script string",
        description: "Bitmap resource name passed to DCProcPreloadBmp.",
    },
    NativeParameterSpec {
        name: "context",
        kind: "script string",
        description: "Converted diagnostic/context string not passed to the target preload core in this build.",
    },
];
const GRAPH92_GET_BITMAP_AUXILIARY_PAIR_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination_pair",
        kind: "BP pointer to 2 DWORDs",
        description: "Receives auxiliary reference-point X/Y from bitmap-registry offsets +0x28/+0x2C.",
    },
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap handle",
        description: "Descriptor queried by sub_402470.",
    },
];
const GRAPH92_READ_BITMAP_PIXEL_VALUE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination_value",
        kind: "BP pointer to DWORD",
        description: "Cleared before the target copies the native one-to-four-byte pixel value.",
    },
    NativeParameterSpec {
        name: "bitmap",
        kind: "bitmap handle",
        description: "Source descriptor.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "i32",
        description: "Pixel X coordinate.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32",
        description: "Pixel Y coordinate.",
    },
];
const GRAPH92_INVERT_ALPHA_BITMAP_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "bitmap",
    kind: "format-3 bitmap handle",
    description: "Alpha/grayscale bytes inverted in place.",
}];
const GRAPH92_COMPOSE_ALPHA_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "destination_gray", kind: "bitmap handle", description: "Required native bitmap slot." },
    NativeParameterSpec { name: "x", kind: "i32", description: "Native positional integer." },
    NativeParameterSpec { name: "y", kind: "i32", description: "Native positional integer." },
    NativeParameterSpec { name: "primary", kind: "bitmap handle", description: "Required native bitmap slot." },
    NativeParameterSpec { name: "other", kind: "bitmap or -1", description: "Optional equal-format second alpha source." },
    NativeParameterSpec { name: "transition", kind: "i32", description: "Native positional integer." },
];
const GRAPH92_BITMAP_TEXT_MEASURE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "destination_bitmap", kind: "bitmap handle", description: "Required native bitmap slot." },
    NativeParameterSpec { name: "x", kind: "i32", description: "Native positional integer." },
    NativeParameterSpec { name: "y", kind: "i32", description: "Native positional integer." },
    NativeParameterSpec { name: "text", kind: "script string", description: "Plain SJIS; LF only layout control." },
    NativeParameterSpec { name: "font", kind: "registered font", description: "Font selector." },
    NativeParameterSpec { name: "height", kind: "i32 4..200", description: "Font pixel height." },
    NativeParameterSpec { name: "width_percent", kind: "i32 25..200", description: "Horizontal cell scaling." },
    NativeParameterSpec { name: "bold", kind: "BOOL", description: "Bold glyph stroke." },
    NativeParameterSpec { name: "proportional", kind: "BOOL", description: "Ink bounds plus native margins." },
    NativeParameterSpec { name: "color", kind: "packed RGB", description: "Glyph color." },
];
const GRAPH92_BITMAP_TEXT_WRAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "destination_bitmap", kind: "bitmap handle", description: "Required native bitmap slot." },
    NativeParameterSpec { name: "x", kind: "i32", description: "Native positional integer." },
    NativeParameterSpec { name: "y", kind: "i32", description: "Native positional integer." },
    NativeParameterSpec { name: "text", kind: "script string", description: "Plain SJIS; LF only layout control." },
    NativeParameterSpec { name: "font", kind: "registered font", description: "Font selector." },
    NativeParameterSpec { name: "height", kind: "i32 4..200", description: "Font pixel height." },
    NativeParameterSpec { name: "width_percent", kind: "i32 25..200", description: "Horizontal cell scaling." },
    NativeParameterSpec { name: "bold", kind: "BOOL", description: "Bold glyph stroke." },
    NativeParameterSpec { name: "proportional", kind: "BOOL", description: "Ink bounds plus native margins." },
    NativeParameterSpec { name: "color", kind: "packed RGB", description: "Glyph color." },
    NativeParameterSpec { name: "extra_line_pitch_percent", kind: "i32 percent", description: "Line advance is height plus height times this percentage." },
];
const GRAPH92_DRAW_BITMAP_TEXT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination_bitmap",
        kind: "bitmap handle",
        description: "Bitmap receiving immediate text.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "i32",
        description: "Text origin X.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32",
        description: "Text origin Y.",
    },
    NativeParameterSpec {
        name: "text",
        kind: "script string",
        description: "Text rendered by the target native font subsystem.",
    },
    NativeParameterSpec {
        name: "font",
        kind: "font selector",
        description: "Target font selector.",
    },
    NativeParameterSpec {
        name: "size",
        kind: "i32",
        description: "Font size.",
    },
    NativeParameterSpec {
        name: "style",
        kind: "i32",
        description: "Font style flags.",
    },
    NativeParameterSpec {
        name: "character_spacing",
        kind: "i32",
        description: "Character advance adjustment.",
    },
    NativeParameterSpec {
        name: "packed_rgb",
        kind: "packed RGB",
        description: "Text color.",
    },
];
const GRAPH92_LOAD_EXTERNAL_BMP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination_bitmap",
        kind: "bitmap handle",
        description: "Descriptor receiving the parsed BMP.",
    },
    NativeParameterSpec {
        name: "resource_name",
        kind: "script string",
        description: "Resource/filesystem name searched by the target loader.",
    },
];

const GRAPH_CONVERT_BITMAP_TO_ALPHA_DESCRIPTOR_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination_descriptor",
        kind: "bitmap handle",
        description: "First BP argument; created/resized as a format-3 bitmap descriptor.",
    },
    NativeParameterSpec {
        name: "source_descriptor",
        kind: "bitmap handle",
        description: "Second BP argument; format-1 RGB or format-2 RGBA source copied/converted by sub_408320.",
    },
];
const MESSAGE_INPUT_SCOPE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "mode",
        kind: "i32 enum 0..=2",
        description: "Target dword_565BA4. Mode 0 uses special scope 2, mode 1 uses scope_value, mode 2 derives the scope from the display object.",
    },
    NativeParameterSpec {
        name: "scope_value",
        kind: "u16-compatible i32",
        description: "Used only by mode 1 and rejected when >= 0x10000.",
    },
];
const MESSAGE_INPUT_FILTER_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "enabled",
    kind: "boolean i32",
    description: "Target dword_565BAC; zero permits the CProcedure host-notify base path.",
}];
const MESSAGE_VARIANT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "surface",
        kind: "graph surface handle",
        description: "CDspObjWindow resolved by sub_4406C0.",
    },
    NativeParameterSpec {
        name: "variant",
        kind: "i32 enum 0..=1",
        description: "Stored at CDspObjWindow+880; value 1 selects CProcDspMsgExVE in Graph92:90.",
    },
];
const SWITCH_PROGRAM_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "program",
    kind: "program/thread handle",
    description: "Coroutine made runnable before the caller yields.",
}];
const QUERY_INPUT_EVENT_BITS_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "scope",
    kind: "i32 registration scope",
    description: "Selects the configured input-event class; the native handler returns the current event bitmap.",
}];
const MESSAGE_AUXILIARY_INPUT_MASK_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "mask",
    kind: "i32 bit mask",
    description: "Stored in target dword_507690 and ORed into the message/procedure input event mask.",
}];
const QUERY_INPUT_CLASS_LEVEL_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "class_mask",
    kind: "i32 input-class mask",
    description: "Selects the input class whose current level/count is queried.",
}];
const QUERY_SCOPED_INPUT_EVENT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "input_descriptor",
        kind: "i32 target logical input descriptor",
        description: "Descriptors 1 and 2 use the pointer-region registry; all other descriptors use the keyboard/global registry.",
    },
    NativeParameterSpec {
        name: "scope",
        kind: "i32 registration scope",
        description: "Packed as (scope << 16) | 0xFFFF before the target registration lookup.",
    },
];
const CONFIGURE_CURSOR_MOTION_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "cancel_on_large_move",
        kind: "bool/i32",
        description: "Cancels the motion when physical cursor displacement exceeds the target logical-to-surface scale tolerance from the last generated point.",
    },
    NativeParameterSpec {
        name: "updates_per_second",
        kind: "u32",
        description: "Combined with duration_ms to derive the target interpolation step count; at least one step is always used.",
    },
    NativeParameterSpec {
        name: "duration_ms",
        kind: "u32 milliseconds",
        description: "Total target cursor-motion duration.",
    },
    NativeParameterSpec {
        name: "interpolation_mode",
        kind: "i32 enum",
        description: "Value 1 selects the target cosine ease; every other value selects linear interpolation.",
    },
    NativeParameterSpec {
        name: "target_y",
        kind: "i32 logical pixels",
        description: "Destination cursor Y coordinate.",
    },
    NativeParameterSpec {
        name: "target_x",
        kind: "i32 logical pixels",
        description: "Destination cursor X coordinate.",
    },
];
const REGISTER_INPUT_CLASS_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "class_mask",
        kind: "i32 target input-class bit",
        description: "One of the target class masks accepted by sub_46DFA0.",
    },
    NativeParameterSpec {
        name: "descriptors",
        kind: "pointer to zero-terminated i32 array",
        description: "At most 15 nonzero input descriptors followed by a zero terminator.",
    },
];
const ENQUEUE_MESSAGE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "thread_or_program",
        kind: "thread/program handle",
        description: "Target thread whose FIFO receives the message.",
    },
    NativeParameterSpec {
        name: "message",
        kind: "DWORD/BP value",
        description: "Single value appended to the target thread FIFO.",
    },
];
const DEQUEUE_MESSAGE_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "output_ptr",
    kind: "BP pointer to DWORD/value",
    description: "Receives one dequeued message when the FIFO is non-empty.",
}];
const ENQUEUE_MESSAGE_ARRAY_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "thread_or_program",
        kind: "thread/program handle",
        description: "Target thread whose FIFO receives the values.",
    },
    NativeParameterSpec {
        name: "message_count",
        kind: "i32 count",
        description: "Number of DWORD/value elements at messages_ptr.",
    },
    NativeParameterSpec {
        name: "messages_ptr",
        kind: "BP pointer to DWORD/value array",
        description: "Source array appended in order.",
    },
];
const DEQUEUE_MESSAGE_ARRAY_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "max_count",
        kind: "i32 count",
        description: "Maximum number of values to dequeue.",
    },
    NativeParameterSpec {
        name: "output_ptr",
        kind: "BP pointer to DWORD/value array",
        description: "Receives up to max_count FIFO values.",
    },
];
const THREAD_CALLBACK_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "thread_or_program",
        kind: "thread/program handle",
        description: "Thread whose callback object is invoked.",
    },
    NativeParameterSpec {
        name: "arg1",
        kind: "DWORD/BP value",
        description: "First callback value.",
    },
    NativeParameterSpec {
        name: "arg2",
        kind: "DWORD/BP value",
        description: "Second callback value.",
    },
    NativeParameterSpec {
        name: "arg3",
        kind: "DWORD/BP value",
        description: "Third callback value.",
    },
];
const SYS_ENCODE_DATA_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "BP pointer",
        description: "Receives the SDC-compressed byte stream.",
    },
    NativeParameterSpec {
        name: "source",
        kind: "BP pointer",
        description: "Source byte buffer.",
    },
    NativeParameterSpec {
        name: "length",
        kind: "u32 bytes",
        description: "Number of source bytes to encode.",
    },
];

const SYS_DECODE_SDC_BUFFER_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "BP pointer",
        description: "Receives decoded bytes.",
    },
    NativeParameterSpec {
        name: "source",
        kind: "BP pointer",
        description: "SDC source stream.",
    },
];

const SYS_ENCODE_STRUCT_ARRAY_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "BP pointer",
        description: "Receives DCFS followed by SDC encoded bytes.",
    },
    NativeParameterSpec {
        name: "source",
        kind: "BP pointer",
        description: "Source fixed-size records.",
    },
    NativeParameterSpec {
        name: "record_size",
        kind: "u32 bytes",
        description: "Size of one source record.",
    },
    NativeParameterSpec {
        name: "record_count",
        kind: "u32",
        description: "Number of records.",
    },
];

const SYS_DECODE_STRUCT_ARRAY_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "BP pointer",
        description: "Receives decoded fixed-size records.",
    },
    NativeParameterSpec {
        name: "source",
        kind: "BP pointer",
        description: "SDC/DCFS encoded source.",
    },
];

const SYS_DECODE_DATA_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "BP pointer",
        description: "Receives decoded data.",
    },
    NativeParameterSpec {
        name: "source",
        kind: "BP pointer",
        description: "Encoded or raw source.",
    },
    NativeParameterSpec {
        name: "requested_length",
        kind: "u32 bytes",
        description: "Requested destination length.",
    },
];

const SYS_RECORD_TABLE_OPEN_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle_out",
        kind: "BP pointer to i32",
        description: "Receives a monotonic integer table handle.",
    },
    NativeParameterSpec {
        name: "record_size",
        kind: "i32 bytes",
        description: "Fixed record size; values <=1 fail.",
    },
];

const SYS_RECORD_TABLE_CLOSE_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "handle",
    kind: "integer handle",
    description: "Table returned by Sys80:D0.",
}];

const SYS_RECORD_TABLE_INSERT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "integer handle",
        description: "Destination table.",
    },
    NativeParameterSpec {
        name: "key",
        kind: "Shift-JIS string",
        description: "Key to insert or replace.",
    },
    NativeParameterSpec {
        name: "source",
        kind: "BP pointer",
        description: "Fixed-size source record.",
    },
];

const SYS_RECORD_TABLE_REMOVE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "integer handle",
        description: "Destination table.",
    },
    NativeParameterSpec {
        name: "key",
        kind: "Shift-JIS string",
        description: "Key to remove.",
    },
];

const SYS_RECORD_TABLE_FETCH_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "BP pointer",
        description: "Receives one fixed-size record.",
    },
    NativeParameterSpec {
        name: "handle",
        kind: "integer handle",
        description: "Source table.",
    },
    NativeParameterSpec {
        name: "key",
        kind: "nullable Shift-JIS pointer",
        description: "Non-null selects by key; null selects by index.",
    },
    NativeParameterSpec {
        name: "index",
        kind: "i32",
        description: "Ordinal used when key is null.",
    },
];

const SYS_STRING_NAMESPACE_COUNT_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "table_id",
    kind: "i32",
    description: "Namespace identifier.",
}];

const SYS_REPLACE_STRING_NAMESPACE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "table_id",
        kind: "i32",
        description: "Namespace identifier.",
    },
    NativeParameterSpec {
        name: "count",
        kind: "i32",
        description: "Zero removes the namespace; positive values load packed strings.",
    },
    NativeParameterSpec {
        name: "source",
        kind: "BP pointer",
        description: "Packed NUL-terminated Shift-JIS strings.",
    },
];

const SYS_SERIALIZE_STRING_NAMESPACE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "nullable BP pointer",
        description: "Receives packed NUL-terminated strings; null performs a size query.",
    },
    NativeParameterSpec {
        name: "table_id",
        kind: "i32",
        description: "Namespace identifier.",
    },
];

const SYS_INTERN_STRING_NAMESPACE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "table_id",
        kind: "i32",
        description: "Namespace identifier.",
    },
    NativeParameterSpec {
        name: "value",
        kind: "Shift-JIS string",
        description: "String to intern.",
    },
];

const SYS_STRING_NAMESPACE_ENTRY_LENGTH_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "length_out",
        kind: "BP pointer to i32",
        description: "Receives Shift-JIS byte length excluding NUL.",
    },
    NativeParameterSpec {
        name: "table_id",
        kind: "i32",
        description: "Namespace identifier.",
    },
    NativeParameterSpec {
        name: "index",
        kind: "i32",
        description: "Zero-based entry index.",
    },
];

const SYS_LAUNCH_PROCESS_WAIT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "working_directory",
        kind: "Shift-JIS string",
        description: "Child working directory.",
    },
    NativeParameterSpec {
        name: "command_line",
        kind: "Shift-JIS string",
        description: "Executable and arguments.",
    },
    NativeParameterSpec {
        name: "error_message",
        kind: "Shift-JIS string",
        description: "Target failure message.",
    },
    NativeParameterSpec {
        name: "restore_parent",
        kind: "bool/i32",
        description: "Whether the parent window is restored after waiting.",
    },
];

const SYS_RESTART_WITH_COMMAND_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "working_directory",
        kind: "nullable Shift-JIS pointer",
        description: "Restart working directory; null selects the target default.",
    },
    NativeParameterSpec {
        name: "command_line",
        kind: "non-null Shift-JIS pointer",
        description: "Command executed after shutdown; a null pointer is fatal, while an empty string is valid.",
    },
    NativeParameterSpec {
        name: "error_message",
        kind: "nullable Shift-JIS pointer",
        description: "Optional launch failure message.",
    },
];

const SYS_LAUNCH_PROCESS_WAIT_UNINSTALLER_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "working_directory",
        kind: "Shift-JIS string",
        description: "Child working directory.",
    },
    NativeParameterSpec {
        name: "command_line",
        kind: "Shift-JIS string",
        description: "Executable and arguments.",
    },
    NativeParameterSpec {
        name: "error_message",
        kind: "Shift-JIS string",
        description: "Target failure message.",
    },
];

const SYS_SHELL_OPEN_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "target",
    kind: "Shift-JIS string",
    description: "Path or URL passed to the platform shell open operation.",
}];

const SYS_WRITE_GAME_ID_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "destination",
    kind: "BP pointer",
    description: "Receives the NUL-terminated game identifier.",
}];

const SYS_HASH_FILE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "hash_out",
        kind: "BP pointer to 8 bytes",
        description: "Receives the target rolling hash state.",
    },
    NativeParameterSpec {
        name: "path",
        kind: "Shift-JIS string",
        description: "Absolute path or path relative to the configured root.",
    },
];

const SYS_SET_UNINSTALLER_PRODUCT_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "product",
    kind: "Shift-JIS string",
    description: "Inserted into the target uninstaller mutex/message template.",
}];

const SYS_SHOW_INPUT_DIALOG_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "initial_text",
        kind: "Shift-JIS string",
        description: "Initial editable text.",
    },
    NativeParameterSpec {
        name: "output_value1",
        kind: "BP pointer to i32",
        description: "Receives the first numeric result.",
    },
    NativeParameterSpec {
        name: "output_value2",
        kind: "BP pointer to i32",
        description: "Receives the second numeric result.",
    },
    NativeParameterSpec {
        name: "output_text",
        kind: "BP character buffer",
        description: "Receives edited text.",
    },
    NativeParameterSpec {
        name: "option_value1",
        kind: "i32",
        description: "First integer option passed to the target modal helper.",
    },
    NativeParameterSpec {
        name: "option_value2",
        kind: "i32",
        description: "Second integer option passed to the target modal helper.",
    },
    NativeParameterSpec {
        name: "mode",
        kind: "i32",
        description: "Dialog mode.",
    },
    NativeParameterSpec {
        name: "caption",
        kind: "Shift-JIS string",
        description: "Dialog caption.",
    },
];

const SYS_SHOW_INSTALLER_DIALOG_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "text1",
        kind: "Shift-JIS string",
        description: "First installer-dialog text field (sub_472050 a2).",
    },
    NativeParameterSpec {
        name: "text2",
        kind: "nullable Shift-JIS string",
        description: "Second text field; selects the target dialog resource variant (sub_472050 a1).",
    },
    NativeParameterSpec {
        name: "text3",
        kind: "Shift-JIS string",
        description: "Third installer-dialog text field.",
    },
    NativeParameterSpec {
        name: "text4",
        kind: "Shift-JIS string",
        description: "Fourth installer-dialog text field.",
    },
    NativeParameterSpec {
        name: "mode1",
        kind: "i32",
        description: "First target installer-dialog mode/global.",
    },
    NativeParameterSpec {
        name: "mode2",
        kind: "i32",
        description: "Second target installer-dialog mode/global.",
    },
];

const SYS_RUN_INSTALLER_WORKFLOW_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "root",
        kind: "Shift-JIS installation path",
        description: "Installation root used by sub_472150/sub_46FB90.",
    },
    NativeParameterSpec {
        name: "optional_directories",
        kind: "nullable zero-terminated string-pointer list",
        description: "Optional directory list copied from BP memory.",
    },
    NativeParameterSpec {
        name: "required_directories",
        kind: "zero-terminated string-pointer list",
        description: "Required directory list copied from BP memory.",
    },
    NativeParameterSpec {
        name: "file_count",
        kind: "i32",
        description: "Count for the parallel source/destination pointer arrays.",
    },
    NativeParameterSpec {
        name: "metadata",
        kind: "Shift-JIS string/pointer",
        description: "Installer metadata passed to the target dialog context.",
    },
    NativeParameterSpec {
        name: "source_files",
        kind: "pointer to file_count Shift-JIS pointers",
        description: "Parallel source-file list.",
    },
    NativeParameterSpec {
        name: "destination_files",
        kind: "pointer to file_count Shift-JIS pointers",
        description: "Parallel destination-file list.",
    },
    NativeParameterSpec {
        name: "flags",
        kind: "i32",
        description: "Installer/uninstall-list flags.",
    },
    NativeParameterSpec {
        name: "format_string",
        kind: "Shift-JIS string",
        description: "Optional protected-file format string used by the target uninstall-list writer.",
    },
    NativeParameterSpec {
        name: "vendor",
        kind: "Shift-JIS string",
        description: "InstalledFolder registry vendor component.",
    },
    NativeParameterSpec {
        name: "product",
        kind: "Shift-JIS string",
        description: "InstalledFolder registry product component and uninstall display class.",
    },
    NativeParameterSpec {
        name: "uninstall_source",
        kind: "nullable Shift-JIS string",
        description: "Uninstaller source/resource name.",
    },
    NativeParameterSpec {
        name: "prompt",
        kind: "Shift-JIS string",
        description: "Installer retry/failure prompt.",
    },
    NativeParameterSpec {
        name: "mode",
        kind: "i32",
        description: "Target installer dialog/workflow mode.",
    },
];

const SYS_RUN_SHORTCUT_INSTALLER_WORKFLOW_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "target_root",
        kind: "Shift-JIS path",
        description: "Root prepended to primary and secondary shortcut targets.",
    },
    NativeParameterSpec {
        name: "primary_target",
        kind: "Shift-JIS relative path",
        description: "Primary shortcut target below target_root.",
    },
    NativeParameterSpec {
        name: "primary_shortcut_name",
        kind: "Shift-JIS string",
        description: "Desktop/program-menu primary shortcut filename.",
    },
    NativeParameterSpec {
        name: "secondary_target",
        kind: "Shift-JIS relative path",
        description: "Secondary program-menu shortcut target.",
    },
    NativeParameterSpec {
        name: "secondary_shortcut_name",
        kind: "Shift-JIS string",
        description: "Secondary program-menu shortcut filename.",
    },
    NativeParameterSpec {
        name: "program_group",
        kind: "Shift-JIS string",
        description: "Program-menu group directory.",
    },
    NativeParameterSpec {
        name: "create_program_group",
        kind: "bool/i32",
        description: "Create both program-menu shortcuts when nonzero.",
    },
    NativeParameterSpec {
        name: "create_desktop",
        kind: "bool/i32",
        description: "Create the primary desktop shortcut when nonzero.",
    },
];

const SYS_REMOVE_UNINSTALL_LISTED_FILES_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "root",
        kind: "Shift-JIS path",
        description: "Installation root containing uninst.lst.",
    },
    NativeParameterSpec {
        name: "exclusions",
        kind: "pointer to NUL-terminated string-pointer array",
        description: "Entries that must not be removed.",
    },
];

const SYS_APPEND_UNINSTALL_LIST_ENTRIES_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "root",
        kind: "Shift-JIS path",
        description: "Installation root containing uninst.lst.",
    },
    NativeParameterSpec {
        name: "entries",
        kind: "pointer to NUL-terminated string-pointer array",
        description: "Entries appended when missing.",
    },
];

const SYS_REMOVE_INSTALLER_SHORTCUTS_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "file_name",
        kind: "Shift-JIS string",
        description: "Primary shortcut name.",
    },
    NativeParameterSpec {
        name: "secondary_file",
        kind: "Shift-JIS string",
        description: "Secondary shortcut name.",
    },
    NativeParameterSpec {
        name: "program_group",
        kind: "Shift-JIS string",
        description: "Program-menu group.",
    },
    NativeParameterSpec {
        name: "remove_group",
        kind: "bool/i32",
        description: "Whether to remove the group directory.",
    },
];

const SYS_CREATE_SHORTCUT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "program_group",
        kind: "nullable Shift-JIS string",
        description: "Optional program-menu group.",
    },
    NativeParameterSpec {
        name: "shortcut_name",
        kind: "Shift-JIS string",
        description: "Shortcut display/file name.",
    },
    NativeParameterSpec {
        name: "target",
        kind: "Shift-JIS string",
        description: "Shortcut target.",
    },
];

const SYS_READ_INSTALLED_FOLDER_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "output",
        kind: "BP character buffer",
        description: "Receives InstalledFolder.",
    },
    NativeParameterSpec {
        name: "vendor",
        kind: "Shift-JIS string",
        description: "Registry vendor component.",
    },
    NativeParameterSpec {
        name: "product",
        kind: "Shift-JIS string",
        description: "Registry product component.",
    },
];

const SYS_DELETE_INSTALLED_REGISTRY_KEY_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "vendor",
        kind: "Shift-JIS string",
        description: "Registry vendor component.",
    },
    NativeParameterSpec {
        name: "product",
        kind: "Shift-JIS string",
        description: "Registry product component.",
    },
];

const SYS_READ_WINDOWS_PATH_FILE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "output",
        kind: "BP character buffer",
        description: "Receives path text with final backslash stripped.",
    },
    NativeParameterSpec {
        name: "file_name",
        kind: "Shift-JIS string",
        description: "File read below the Windows directory.",
    },
];

const SYS_WRITE_WINDOWS_DIRECTORY_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "output",
    kind: "BP character buffer",
    description: "Receives the Windows directory.",
}];

const SYS_REGISTER_FILE_ASSOCIATION_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "extension",
        kind: "Shift-JIS string",
        description: "File extension.",
    },
    NativeParameterSpec {
        name: "class_name",
        kind: "Shift-JIS string",
        description: "Registry class name.",
    },
    NativeParameterSpec {
        name: "description",
        kind: "Shift-JIS string",
        description: "Human-readable description.",
    },
    NativeParameterSpec {
        name: "icon",
        kind: "Shift-JIS string",
        description: "Default icon specification.",
    },
    NativeParameterSpec {
        name: "open_command",
        kind: "Shift-JIS string",
        description: "Shell open command.",
    },
];

const COPY_INDEXED_NAMESPACE_RECORD_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "BP pointer to byte buffer",
        description: "Receives the selected entry payload, including its terminating NUL byte.",
    },
    NativeParameterSpec {
        name: "table_id",
        kind: "i32 namespace identifier",
        description: "Selects the native namespace table.",
    },
    NativeParameterSpec {
        name: "index",
        kind: "i32 record index",
        description: "Zero-based entry index copied to destination.",
    },
];
const RESOURCE_NAME_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "name",
    kind: "Shift-JIS string",
    description: "Resource name interned or queried in the target global resource-name namespace.",
}];
const READ_FLAG_TABLE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "name",
        kind: "Shift-JIS string",
        description: "Name of the read-flag table.",
    },
    NativeParameterSpec {
        name: "bit_count",
        kind: "i32 bit count",
        description: "Requested table size in bits; resize preserves the overlapping prefix.",
    },
];
const READ_FLAG_BIT_SET_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "name",
        kind: "Shift-JIS string",
        description: "Name of the read-flag table.",
    },
    NativeParameterSpec {
        name: "bit_offset",
        kind: "i32 bit index",
        description: "Zero-based flag index.",
    },
    NativeParameterSpec {
        name: "enabled",
        kind: "bool/i32",
        description: "Nonzero sets the bit; zero clears it.",
    },
];
const READ_FLAG_RANGE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "name",
        kind: "Shift-JIS string",
        description: "Name of the read-flag table.",
    },
    NativeParameterSpec {
        name: "start_bit",
        kind: "i32 bit index",
        description: "Zero-based first bit in the range.",
    },
    NativeParameterSpec {
        name: "enabled",
        kind: "bool/i32",
        description: "Nonzero sets the range; zero clears it.",
    },
    NativeParameterSpec {
        name: "bit_count",
        kind: "i32 bit count",
        description: "Number of consecutive bits to modify.",
    },
];
const READ_FLAG_QUERY_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "name",
        kind: "Shift-JIS string",
        description: "Name of the read-flag table.",
    },
    NativeParameterSpec {
        name: "output_ptr",
        kind: "BP pointer to i32",
        description: "Receives zero or one for the queried bit.",
    },
    NativeParameterSpec {
        name: "bit_offset",
        kind: "i32 bit index",
        description: "Zero-based flag index.",
    },
];
const GLOBAL_DATA_BLOCK_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "offset",
        kind: "i32 byte offset",
        description: "Offset into the fixed 1 MiB global data region.",
    },
    NativeParameterSpec {
        name: "buffer",
        kind: "BP pointer",
        description: "Source for writes or destination for reads.",
    },
    NativeParameterSpec {
        name: "length",
        kind: "i32 byte count",
        description: "Nonzero transfer length bounded by the 1 MiB region.",
    },
];
const STRUCTURED_HISTORY_CAPACITY_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "capacity",
    kind: "i32 record count",
    description: "Resets the process-global newest-first history and sets its bounded capacity.",
}];
const STRUCTURED_HISTORY_FIELDS_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "value0",
        kind: "i32",
        description: "Native scalar field 0.",
    },
    NativeParameterSpec {
        name: "value1",
        kind: "i32",
        description: "Native scalar field 1.",
    },
    NativeParameterSpec {
        name: "value2",
        kind: "i32",
        description: "Native scalar field 2.",
    },
    NativeParameterSpec {
        name: "value3",
        kind: "i32",
        description: "Native scalar field 3.",
    },
    NativeParameterSpec {
        name: "value4",
        kind: "i32",
        description: "Native scalar field 4.",
    },
    NativeParameterSpec {
        name: "value5",
        kind: "i32",
        description: "Native scalar field 5.",
    },
    NativeParameterSpec {
        name: "value6",
        kind: "i32",
        description: "Native scalar field 6.",
    },
    NativeParameterSpec {
        name: "value7",
        kind: "i32",
        description: "Native scalar field 7.",
    },
    NativeParameterSpec {
        name: "value8",
        kind: "i32",
        description: "Native scalar field 8.",
    },
    NativeParameterSpec {
        name: "short_text1",
        kind: "Shift-JIS string",
        description: "First optional Shift-JIS string, limited to 31 bytes.",
    },
    NativeParameterSpec {
        name: "short_text2",
        kind: "Shift-JIS string",
        description: "Second optional Shift-JIS string, limited to 31 bytes.",
    },
    NativeParameterSpec {
        name: "short_text3",
        kind: "Shift-JIS string",
        description: "Third optional Shift-JIS string, limited to 31 bytes.",
    },
    NativeParameterSpec {
        name: "text",
        kind: "Shift-JIS string",
        description: "Required Shift-JIS text, limited to 255 bytes.",
    },
];
const STRUCTURED_HISTORY_READ_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "output_ptr",
        kind: "BP pointer to record",
        description: "Receives the target 0x200/extended record layout.",
    },
    NativeParameterSpec {
        name: "newest_index",
        kind: "u32 index",
        description: "Zero selects the newest record and follows native prev links for older entries.",
    },
];
const STRUCTURED_HISTORY_RECORD_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "record_ptr",
    kind: "BP pointer to record",
    description: "Points to scalar fields at 0/64..92 and strings at 160/192/224/256/512.",
}];
const INDEXED_RECORD_OPEN_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle_output",
        kind: "BP pointer to i32",
        description: "Receives the monotonically allocated native table identifier.",
    },
    NativeParameterSpec {
        name: "capacity",
        kind: "u32 record count",
        description: "Maximum retained records.",
    },
    NativeParameterSpec {
        name: "record_size",
        kind: "u32 byte count",
        description: "Fixed bytes copied for each record.",
    },
];
const INDEXED_RECORD_HANDLE_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "handle",
    kind: "i32 table identifier",
    description: "Previously returned indexed-record table identifier.",
}];
const INDEXED_RECORD_COUNT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "output_ptr",
        kind: "BP pointer to i32",
        description: "Receives the current record count.",
    },
    NativeParameterSpec {
        name: "handle",
        kind: "i32 table identifier",
        description: "Indexed-record table identifier.",
    },
];
const INDEXED_RECORD_PUSH_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "i32 table identifier",
        description: "Indexed-record table identifier.",
    },
    NativeParameterSpec {
        name: "source",
        kind: "BP pointer",
        description: "Source record copied to the newest/front position.",
    },
];
const INDEXED_RECORD_LOAD_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "output_ptr",
        kind: "BP pointer",
        description: "Receives the selected fixed-size record.",
    },
    NativeParameterSpec {
        name: "handle",
        kind: "i32 table identifier",
        description: "Indexed-record table identifier.",
    },
    NativeParameterSpec {
        name: "index",
        kind: "u32 newest-first index",
        description: "Zero selects the most recently pushed record.",
    },
];
const INDEXED_RECORD_REMOVE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "i32 table identifier",
        description: "Indexed-record table identifier.",
    },
    NativeParameterSpec {
        name: "start",
        kind: "u32 newest-first index",
        description: "First record removed.",
    },
    NativeParameterSpec {
        name: "count",
        kind: "u32 record count",
        description: "Maximum number of records removed.",
    },
];
const QUEUED_EVENT_POLL_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "output_ptr",
    kind: "BP pointer to three i32 values",
    description: "Receives source, event code and parameter for the oldest process-global event.",
}];
const QUEUED_EVENT_POST_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "event_code",
        kind: "i32",
        description: "Second DWORD of the queued native event.",
    },
    NativeParameterSpec {
        name: "parameter",
        kind: "i32",
        description: "Third DWORD; the first DWORD is always zero.",
    },
];
const REGISTERED_OBJECT_STATE_SET_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "object_id",
        kind: "i32 registry identifier",
        description: "Registered DCIndProc object identifier.",
    },
    NativeParameterSpec {
        name: "state",
        kind: "i32",
        description: "Value written to the registered object state field.",
    },
];
const REGISTERED_OBJECT_STATE_GET_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "object_id",
        kind: "i32 registry identifier",
        description: "Registered DCIndProc object identifier.",
    },
    NativeParameterSpec {
        name: "output_ptr",
        kind: "BP pointer to i32",
        description: "Receives the registered object state.",
    },
];
const REGISTERED_OBJECT_MESSAGE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "object_id",
        kind: "i32 registry identifier",
        description: "Registered object whose variable-length message queue receives the descriptor.",
    },
    NativeParameterSpec {
        name: "count",
        kind: "i32 DWORD count 1..=256",
        description: "Number of descriptor DWORDs copied.",
    },
    NativeParameterSpec {
        name: "descriptor",
        kind: "BP pointer to DWORD array",
        description: "Message values copied into a newly allocated queue node.",
    },
];
const SYSTEM_MODE_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "mode",
    kind: "i32 0..=1",
    description: "Validated process-global system mode flag.",
}];
const EXCLUSION_CREATE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "name",
        kind: "Shift-JIS string",
        description: "Process-global exclusion section name.",
    },
    NativeParameterSpec {
        name: "capacity",
        kind: "i32",
        description: "Maximum simultaneous holders.",
    },
];
const EXCLUSION_NAME_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "name",
    kind: "Shift-JIS string",
    description: "Process-global exclusion section name.",
}];
const EXCLUSION_PRIORITY_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "name",
        kind: "Shift-JIS string",
        description: "Process-global exclusion section name.",
    },
    NativeParameterSpec {
        name: "priority",
        kind: "u32",
        description: "Higher priorities are inserted before lower priorities; equal priorities remain FIFO.",
    },
];

const MAIN_LOOP_WAIT_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "value",
    kind: "i32",
    description: "Stored unchanged in dword_503EF8 and consulted by the target main-loop pacing branch.",
}];
const EXCLUSIVE_THREAD_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "enabled",
    kind: "bool/i32",
    description: "Nonzero stores the current CThread pointer and enables exclusive scheduling; zero clears both globals.",
}];
const DISPLAY_MODE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "mode",
        kind: "i32 display mode 0..=7",
        description: "Indexes the target display-mode table.",
    },
    NativeParameterSpec {
        name: "adapter",
        kind: "i32 adapter 0..=1",
        description: "Selects one of the two native adapters.",
    },
    NativeParameterSpec {
        name: "fullscreen",
        kind: "bool/i32",
        description: "Nonzero selects the fullscreen window path.",
    },
];
const FULLSCREEN_HOTKEY_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "enabled",
        kind: "bool/i32",
        description: "Enables or disables the fullscreen-toggle descriptor filter.",
    },
    NativeParameterSpec {
        name: "descriptor_list",
        kind: "BP pointer to zero-terminated i32 list",
        description: "At most fifteen nonzero descriptors followed by zero.",
    },
];
const BOOL_VALUE_PARAMETER: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "enabled",
    kind: "bool/i32",
    description: "Zero disables the target state; nonzero enables it.",
}];
const STRING_VALUE_PARAMETER: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "text",
    kind: "Shift-JIS string",
    description: "NUL-terminated target string.",
}];
const CURSOR_INDEX_PARAMETER: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "cursor_index",
    kind: "i32 enum 0..=4",
    description: "Indexes the target hCursor table used by WM_SETCURSOR.",
}];
const BOOTSTRAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "root_or_archive_path",
        kind: "Shift-JIS path",
        description: "Directory, archive path, missing path, or dot interpreted by sub_4650F0.",
    },
    NativeParameterSpec {
        name: "archive_namespace",
        kind: "Shift-JIS string",
        description: "Logical archive namespace associated with system.arc or the selected archive basename.",
    },
];
const OUTPUT_STRING_PARAMETER: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "destination",
    kind: "BP char buffer",
    description: "Receives a NUL-terminated Shift-JIS path when available.",
}];
const GLOBAL_CONFIG_SHIFT_PARAMETER: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "shift",
    kind: "i32 0..=12",
    description: "Allocates and zeroes 4096 << shift bytes.",
}];
const SAVE_SLOT_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "slot",
    kind: "i32",
    description: "Formats as BGI%04d.cad.",
}];
const SAVE_SLOT_WRITE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "slot",
        kind: "i32",
        description: "Formats as BGI%04d.cad.",
    },
    NativeParameterSpec {
        name: "label",
        kind: "Shift-JIS string shorter than 40 bytes",
        description: "Copied to header offset 16 after SYSTEMTIME.",
    },
];
const SAVE_SLOT_HEADER_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "BP pointer to 64 bytes",
        description: "Receives the file header only when the full slot length is present.",
    },
    NativeParameterSpec {
        name: "slot",
        kind: "i32",
        description: "Formats as BGI%04d.cad.",
    },
];

const WAIT_TIMING_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "duration_ms",
        kind: "i32 milliseconds",
        description: "Minimum wait duration measured on the native engine clock.",
    },
    NativeParameterSpec {
        name: "input_enabled",
        kind: "bool/i32",
        description: "Non-zero permits an input event to interrupt the wait.",
    },
    NativeParameterSpec {
        name: "input_scope",
        kind: "i32 mask",
        description: "Input class queried while the wait procedure is active.",
    },
];

const SYS81_04_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "threshold_ms",
    kind: "i32 milliseconds",
    description: "Accepted range 50..=60000; stored in dword_503DE8.",
}];

const SYS81_07_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "index",
        kind: "i32",
        description: "Coordinate slot 0..4.",
    },
    NativeParameterSpec {
        name: "destination",
        kind: "pointer to two i32",
        description: "Receives the transformed X/Y pair.",
    },
];

const SYS81_08_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "destination",
    kind: "Shift-JIS buffer",
    description: "Receives GetUserNameA output.",
}];

const SYS81_09_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "destination",
    kind: "Shift-JIS buffer",
    description: "Receives GetComputerNameA output.",
}];

const SYS81_0A_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "destination",
    kind: "Shift-JIS buffer",
    description: "Receives the normalized CPUID extended brand string.",
}];

const SYS81_0B_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "reserved",
        kind: "pointer",
        description: "Popped but not consumed by the release helper.",
    },
    NativeParameterSpec {
        name: "signature",
        kind: "pointer to four i32",
        description: "Receives four cached CPU signature words.",
    },
    NativeParameterSpec {
        name: "text",
        kind: "Shift-JIS buffer",
        description: "Normalized in place by the target CPU text helper.",
    },
];

const SYS81_0C_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "service_pack",
        kind: "Shift-JIS buffer",
        description: "Receives OSVERSIONINFOA.szCSDVersion.",
    },
    NativeParameterSpec {
        name: "version",
        kind: "pointer to four u32",
        description: "Receives major, minor, build and platform ID.",
    },
];

const SYS81_0D_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "available_mb",
        kind: "pointer to u32",
        description: "Receives available physical memory in MiB.",
    },
    NativeParameterSpec {
        name: "total_mb",
        kind: "pointer to u32",
        description: "Receives total physical memory in MiB.",
    },
];

const SYS81_0E_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "destination",
    kind: "pointer to two u32",
    description: "Receives adjusted width and height.",
}];

const SYS81_10_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "replacement",
        kind: "i32",
        description: "New value stored in the selected six-DWORD input record.",
    },
    NativeParameterSpec {
        name: "index",
        kind: "i32",
        description: "Input record index.",
    },
];

const SYS81_11_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "destination",
    kind: "pointer to 256 bytes",
    description: "Receives GetKeyboardState output.",
}];

const SYS81_14_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "enabled",
    kind: "i32",
    description: "Stored in dword_5667EC.",
}];

const SYS81_16_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "distance",
        kind: "i32",
        description: "Minimum movement distance stored as double.",
    },
    NativeParameterSpec {
        name: "capacity",
        kind: "i32",
        description: "History capacity, maximum 512.",
    },
];

const SYS81_17_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "count",
        kind: "i32",
        description: "Maximum samples to copy.",
    },
    NativeParameterSpec {
        name: "start",
        kind: "i32",
        description: "Newest-first start index.",
    },
    NativeParameterSpec {
        name: "distances",
        kind: "pointer to i32 array",
        description: "Receives distance/angle helper values.",
    },
    NativeParameterSpec {
        name: "points",
        kind: "pointer to x/y pairs",
        description: "Receives sample coordinates.",
    },
];

const SYS81_18_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "enabled",
    kind: "i32",
    description: "Nonzero registers; zero unregisters.",
}];

const SYS81_19_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "destination",
    kind: "pointer to six-DWORD records",
    description: "Receives the current touch records.",
}];

const SYS81_1B_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "value",
        kind: "i32",
        description: "Value stored for the selected entry.",
    },
    NativeParameterSpec {
        name: "index",
        kind: "i32",
        description: "Controller wake entry 0..35.",
    },
];

const SYS81_1D_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "pointer to six i32",
        description: "Receives aggregated axes, POV and button mask.",
    },
    NativeParameterSpec {
        name: "device",
        kind: "pointer/nullable device selector",
        description: "Null aggregates all matching devices.",
    },
];

const SYS81_1E_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "button_code",
    kind: "i32",
    description: "1 left, 2 right, 4 middle, 5 X1, 6 X2.",
}];

const SYS81_28_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "mode",
        kind: "i32",
        description: "0 read, 1 write/create, 2 alternate write mode.",
    },
    NativeParameterSpec {
        name: "path",
        kind: "Shift-JIS path",
        description: "Canonicalized resource path.",
    },
    NativeParameterSpec {
        name: "handle_out",
        kind: "pointer to u32",
        description: "Receives integer stream handle.",
    },
];

const SYS81_29_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "integer stream handle",
        description: "Stream selected for queued close.",
    },
    NativeParameterSpec {
        name: "status_out",
        kind: "pointer to i32",
        description: "Worker writes completion status.",
    },
];

const SYS81_2A_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "length",
        kind: "i32 bytes",
        description: "Requested read count.",
    },
    NativeParameterSpec {
        name: "destination",
        kind: "byte buffer",
        description: "Receives bytes.",
    },
    NativeParameterSpec {
        name: "handle",
        kind: "integer stream handle",
        description: "Open stream.",
    },
    NativeParameterSpec {
        name: "status_out",
        kind: "pointer to i32",
        description: "Worker completion/count field.",
    },
];

const SYS81_2B_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "offset",
        kind: "i32",
        description: "New stream offset.",
    },
    NativeParameterSpec {
        name: "handle",
        kind: "integer stream handle",
        description: "Open stream.",
    },
    NativeParameterSpec {
        name: "status_out",
        kind: "pointer to i32",
        description: "Worker completion field.",
    },
];

const SYS81_2C_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "path",
        kind: "Shift-JIS path",
        description: "File/resource path.",
    },
    NativeParameterSpec {
        name: "write_time",
        kind: "SYSTEMTIME pointer",
        description: "Receives last-write time.",
    },
    NativeParameterSpec {
        name: "access_time",
        kind: "SYSTEMTIME pointer",
        description: "Receives last-access time.",
    },
    NativeParameterSpec {
        name: "creation_time",
        kind: "SYSTEMTIME pointer",
        description: "Receives creation time.",
    },
];

const SYS81_2D_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "write_time",
        kind: "SYSTEMTIME pointer",
        description: "New last-write time.",
    },
    NativeParameterSpec {
        name: "access_time",
        kind: "SYSTEMTIME pointer",
        description: "New last-access time.",
    },
    NativeParameterSpec {
        name: "creation_time",
        kind: "SYSTEMTIME pointer",
        description: "New creation time.",
    },
    NativeParameterSpec {
        name: "path",
        kind: "Shift-JIS path",
        description: "File path passed through the target hidden register.",
    },
];

const SYS81_2F_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "directory",
    kind: "Shift-JIS path",
    description: "Directory tested using a temporary BGI-prefixed file.",
}];

const SYS81_30_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "byte buffer",
        description: "Receives data.",
    },
    NativeParameterSpec {
        name: "archive",
        kind: "Shift-JIS archive/root",
        description: "Resource namespace or archive container.",
    },
    NativeParameterSpec {
        name: "file",
        kind: "Shift-JIS file name",
        description: "Resource entry name; empty selects the first archive entry.",
    },
    NativeParameterSpec {
        name: "offset",
        kind: "i32 bytes",
        description: "Source offset.",
    },
    NativeParameterSpec {
        name: "length",
        kind: "i32 bytes",
        description: "Requested byte count or destination capacity for the default archive entry.",
    },
];

const SYS81_31_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "length",
        kind: "i32 bytes",
        description: "Requested byte count.",
    },
    NativeParameterSpec {
        name: "offset",
        kind: "i32 bytes",
        description: "HTTP stream offset.",
    },
    NativeParameterSpec {
        name: "url",
        kind: "Shift-JIS URL",
        description: "Internet resource.",
    },
    NativeParameterSpec {
        name: "destination",
        kind: "buffer/size pointer",
        description: "Receives bytes or queried size.",
    },
];

const SYS81_32_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "bytes_read",
        kind: "pointer to u32",
        description: "Receives actual byte count.",
    },
    NativeParameterSpec {
        name: "path",
        kind: "Shift-JIS path",
        description: "Physical drive-letter path.",
    },
    NativeParameterSpec {
        name: "destination",
        kind: "byte buffer",
        description: "Receives data.",
    },
    NativeParameterSpec {
        name: "length",
        kind: "i32 bytes",
        description: "Requested byte count.",
    },
];

const SYS81_35_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "path",
        kind: "Shift-JIS path",
        description: "Resource name.",
    },
    NativeParameterSpec {
        name: "context",
        kind: "pointer/reserved",
        description: "Popped after the path and ignored by the target wrapper.",
    },
];

const SYS81_36_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "destination",
    kind: "pointer to 26 i32",
    description: "Receives target drive-type codes A..Z.",
}];

const SYS81_37_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "path",
        kind: "Shift-JIS path",
        description: "Volume/directory path.",
    },
    NativeParameterSpec {
        name: "destination",
        kind: "pointer to u32",
        description: "Receives free MiB.",
    },
];

const SYS81_38_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "mode",
        kind: "i32",
        description: "Dialog mode.",
    },
    NativeParameterSpec {
        name: "title",
        kind: "Shift-JIS text",
        description: "Dialog title.",
    },
    NativeParameterSpec {
        name: "default_name",
        kind: "Shift-JIS text",
        description: "Default selection.",
    },
    NativeParameterSpec {
        name: "extension_table",
        kind: "pointer to DWORD string pointers",
        description: "VM table containing filter extensions/patterns.",
    },
    NativeParameterSpec {
        name: "label_table",
        kind: "pointer to DWORD string pointers",
        description: "VM table containing parallel filter labels.",
    },
    NativeParameterSpec {
        name: "filter_count",
        kind: "i32",
        description: "Number of label/extension entries.",
    },
    NativeParameterSpec {
        name: "destination",
        kind: "Shift-JIS buffer",
        description: "Receives selected path.",
    },
];

const SYS81_39_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "destination",
        kind: "packed NUL strings",
        description: "Optional output buffer.",
    },
    NativeParameterSpec {
        name: "required_bytes",
        kind: "pointer to u32",
        description: "Receives packed byte count.",
    },
    NativeParameterSpec {
        name: "pattern",
        kind: "Shift-JIS pattern",
        description: "Archive/resource wildcard.",
    },
];

const SYS81_3A_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "root",
        kind: "i32/LPARAM",
        description: "Root selector passed to the callback.",
    },
    NativeParameterSpec {
        name: "title",
        kind: "Shift-JIS text",
        description: "Browser title.",
    },
    NativeParameterSpec {
        name: "destination",
        kind: "Shift-JIS buffer",
        description: "Receives selected path.",
    },
];

const SYS81_3B_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "reserved",
        kind: "pointer",
        description: "Popped by wrapper.",
    },
    NativeParameterSpec {
        name: "title",
        kind: "Shift-JIS text",
        description: "Modal list title.",
    },
    NativeParameterSpec {
        name: "pattern",
        kind: "Shift-JIS pattern",
        description: "Resources enumerated into newline text.",
    },
    NativeParameterSpec {
        name: "context",
        kind: "pointer",
        description: "Resource context/root.",
    },
];

const SYS81_3C_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "path",
    kind: "Shift-JIS path",
    description: "Resource path searched across configured roots.",
}];

const SYS81_3D_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "path",
        kind: "Shift-JIS drive/path",
        description: "Volume to query.",
    },
    NativeParameterSpec {
        name: "destination",
        kind: "Shift-JIS buffer",
        description: "Receives volume label.",
    },
];

const SYS81_3E_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "path",
        kind: "Shift-JIS device/file path",
        description: "Object opened for power-state query.",
    },
    NativeParameterSpec {
        name: "destination",
        kind: "pointer to BOOL",
        description: "Receives GetDevicePowerState result.",
    },
];

const SYS81_44_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "data_size",
        kind: "i32",
        description: "Child data region size.",
    },
    NativeParameterSpec {
        name: "code_size",
        kind: "i32",
        description: "Child code region size.",
    },
    NativeParameterSpec {
        name: "operand_slots",
        kind: "i32",
        description: "Child operand-slot count.",
    },
    NativeParameterSpec {
        name: "entry_index",
        kind: "i32",
        description: "Initial child entry/index validated against the new thread.",
    },
];

const SYS81_60_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "second",
        kind: "i32",
        description: "Second dimension/value.",
    },
    NativeParameterSpec {
        name: "first",
        kind: "i32",
        description: "First dimension/value.",
    },
    NativeParameterSpec {
        name: "index",
        kind: "i32",
        description: "One of eight mode slots.",
    },
];

const SYS81_62_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "mode",
    kind: "i32",
    description: "Accepted values 0 or 1.",
}];

const SYS81_63_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "mode",
    kind: "i32",
    description: "Accepted values 0..2.",
}];

const SYS81_64_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "height",
        kind: "i32",
        description: "Logical height; zero selects target default.",
    },
    NativeParameterSpec {
        name: "width",
        kind: "i32",
        description: "Logical width; zero selects target default.",
    },
];

const SYS81_65_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "value",
    kind: "i32",
    description: "Stored in dword_566A60.",
}];

const SYS81_68_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "enabled",
    kind: "i32",
    description: "New focus-deactivation pause flag.",
}];

const SYS81_69_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "enabled",
    kind: "i32",
    description: "Register or unregister modifier combinations.",
}];

const SYS81_6A_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "mode",
    kind: "i32",
    description: "Stored in dword_566000.",
}];

const SYS81_6B_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "destination",
    kind: "Shift-JIS buffer/nullable",
    description: "Receives captured error text.",
}];

const SYS81_6F_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "mode",
    kind: "i32",
    description: "0 disables, 1 enables if initialization succeeds.",
}];

const SYS81_B0_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "left",
        kind: "UTF-16 NUL string",
        description: "First string.",
    },
    NativeParameterSpec {
        name: "right",
        kind: "UTF-16 NUL string",
        description: "Second string.",
    },
];

const SYS81_B7_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "source",
        kind: "Shift-JIS NUL string",
        description: "Input bytes.",
    },
    NativeParameterSpec {
        name: "destination",
        kind: "UTF-16 buffer/nullable",
        description: "Receives converted units and NUL.",
    },
];

const SYS81_D0_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "capacity",
        kind: "i32",
        description: "Initial slot capacity; must exceed one.",
    },
    NativeParameterSpec {
        name: "handle_out",
        kind: "pointer to u32",
        description: "Receives integer table handle.",
    },
];

const SYS81_D1_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "handle",
    kind: "integer table handle",
    description: "Table to destroy.",
}];

const SYS81_D2_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "integer table handle",
        description: "Target table.",
    },
    NativeParameterSpec {
        name: "source",
        kind: "byte buffer",
        description: "Blob bytes.",
    },
    NativeParameterSpec {
        name: "size",
        kind: "i32 bytes",
        description: "Blob length.",
    },
    NativeParameterSpec {
        name: "requested_index",
        kind: "i32",
        description: "Used when index_out is null.",
    },
    NativeParameterSpec {
        name: "index_out",
        kind: "pointer/nullable",
        description: "Non-null requests first-free insertion and receives index.",
    },
];

const SYS81_D3_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "index",
        kind: "i32",
        description: "Slot index.",
    },
    NativeParameterSpec {
        name: "handle",
        kind: "integer table handle",
        description: "Target table.",
    },
];

const SYS81_D4_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "index",
        kind: "i32",
        description: "Slot index.",
    },
    NativeParameterSpec {
        name: "handle",
        kind: "integer table handle",
        description: "Target table.",
    },
    NativeParameterSpec {
        name: "size_out",
        kind: "pointer to u32",
        description: "Receives blob size.",
    },
    NativeParameterSpec {
        name: "destination",
        kind: "byte buffer/nullable",
        description: "Receives blob bytes.",
    },
];

const SYS81_D5_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "integer table handle",
        description: "Target table.",
    },
    NativeParameterSpec {
        name: "count_out",
        kind: "pointer to u32",
        description: "Receives occupied count.",
    },
    NativeParameterSpec {
        name: "pairs_out",
        kind: "pointer to index/size pairs",
        description: "Receives occupied slot descriptors.",
    },
];

const SYS81_E0_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "flags",
        kind: "i32",
        description: "Target launch/show flags.",
    },
    NativeParameterSpec {
        name: "error_message",
        kind: "Shift-JIS text",
        description: "Retry/error prompt.",
    },
    NativeParameterSpec {
        name: "show_window",
        kind: "i32",
        description: "Controls STARTUPINFO show state.",
    },
    NativeParameterSpec {
        name: "working_directory",
        kind: "Shift-JIS path",
        description: "Child working directory.",
    },
    NativeParameterSpec {
        name: "executable",
        kind: "Shift-JIS file",
        description: "Executable path/name.",
    },
    NativeParameterSpec {
        name: "arguments",
        kind: "Shift-JIS command line",
        description: "Child arguments.",
    },
    NativeParameterSpec {
        name: "exit_code_out",
        kind: "pointer to u32/nullable",
        description: "Receives child exit code.",
    },
];

const SYS81_E9_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "length",
        kind: "i32 bytes",
        description: "Number of bytes.",
    },
    NativeParameterSpec {
        name: "source",
        kind: "byte buffer",
        description: "Input bytes.",
    },
    NativeParameterSpec {
        name: "state",
        kind: "pointer to 8 bytes",
        description: "In/out rolling hash state.",
    },
];

const SYS81_EA_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "length",
        kind: "i32 bytes",
        description: "Input length.",
    },
    NativeParameterSpec {
        name: "source",
        kind: "byte buffer",
        description: "Input bytes.",
    },
    NativeParameterSpec {
        name: "destination",
        kind: "pointer to 16 bytes",
        description: "Receives standard MD5 digest.",
    },
];

const SYS81_EC_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "name",
    kind: "Shift-JIS mutex name",
    description: "CreateMutexA name.",
}];

const SYS81_ED_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "handle",
    kind: "integer mutex handle",
    description: "BGI handle returned by 0xEC.",
}];

const SYS81_F2_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "root",
        kind: "Shift-JIS installation path",
        description: "Root passed in EDX to sub_451680 and validated by sub_46FD10.",
    },
    NativeParameterSpec {
        name: "optional_directories",
        kind: "nullable zero-terminated string-pointer list",
        description: "Optional directory list converted by sub_451CF0.",
    },
    NativeParameterSpec {
        name: "required_directories",
        kind: "zero-terminated string-pointer list",
        description: "Required directory list converted by sub_451CF0.",
    },
    NativeParameterSpec {
        name: "file_count",
        kind: "i32",
        description: "Count shared by the descriptor and parallel source/destination arrays.",
    },
    NativeParameterSpec {
        name: "descriptor_values",
        kind: "pointer to file_count DWORDs",
        description: "Per-entry descriptor array copied into DCProcInstallation+0x648.",
    },
    NativeParameterSpec {
        name: "source_files",
        kind: "pointer to file_count Shift-JIS pointers",
        description: "Parallel source-file list converted from BP memory.",
    },
    NativeParameterSpec {
        name: "destination_files",
        kind: "pointer to file_count Shift-JIS pointers",
        description: "Parallel destination-file list converted from BP memory.",
    },
    NativeParameterSpec {
        name: "mode",
        kind: "i32",
        description: "Stored at DCProcInstallation+0x654.",
    },
    NativeParameterSpec {
        name: "text1",
        kind: "Shift-JIS string",
        description: "First installation procedure text copied to the native object.",
    },
    NativeParameterSpec {
        name: "text2",
        kind: "Shift-JIS string",
        description: "Second installation procedure text copied to the native object.",
    },
    NativeParameterSpec {
        name: "text3",
        kind: "Shift-JIS string",
        description: "Third installation procedure text copied to the native object.",
    },
    NativeParameterSpec {
        name: "text4",
        kind: "Shift-JIS string",
        description: "Fourth installation procedure text copied to the native object.",
    },
    NativeParameterSpec {
        name: "text5",
        kind: "Shift-JIS string",
        description: "Fifth installation procedure text copied to the native object.",
    },
];

const SYS81_F7_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "program_group",
        kind: "nullable Shift-JIS string",
        description: "Optional group directory below the selected special folder.",
    },
    NativeParameterSpec {
        name: "shortcut_name",
        kind: "Shift-JIS string",
        description: "Shortcut filename.",
    },
    NativeParameterSpec {
        name: "target",
        kind: "Shift-JIS string",
        description: "Shortcut target passed to IShellLink::SetPath.",
    },
    NativeParameterSpec {
        name: "special_folder_mode",
        kind: "i32",
        description: "Selector forwarded in ECX to sub_467570 before creating the shortcut.",
    },
];

const SOUND_CHANNEL_VOLUME_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "channel",
        kind: "i32 channel index",
        description: "BGM 0..15 or SE 0..63, depending on the selector.",
    },
    NativeParameterSpec {
        name: "volume",
        kind: "i32 0..128",
        description: "Native linear control; 128 is full and 0 is silent before mixer composition.",
    },
];

const SOUND_BGM_FILE_LOAD_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "channel",
        kind: "i32 BGM channel",
        description: "Resident BGM slot 0..15.",
    },
    NativeParameterSpec {
        name: "file",
        kind: "Shift-JIS resource name",
        description: "BGM resource loaded into the resident slot.",
    },
    NativeParameterSpec {
        name: "volume",
        kind: "i32 0..128",
        description: "Initial per-channel BGM volume.",
    },
];

const SOUND_BGM_ARCHIVE_LOAD_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "channel",
        kind: "i32 BGM channel",
        description: "Resident BGM slot 0..15.",
    },
    NativeParameterSpec {
        name: "archive",
        kind: "Shift-JIS archive/root",
        description: "Archive or resource root.",
    },
    NativeParameterSpec {
        name: "file",
        kind: "Shift-JIS resource name",
        description: "BGM stream resource.",
    },
    NativeParameterSpec {
        name: "volume",
        kind: "i32 0..128",
        description: "Initial per-channel BGM volume.",
    },
    NativeParameterSpec {
        name: "pan",
        kind: "i32 0..128",
        description: "Native pan with center 64.",
    },
];

const SOUND_BGM_PAIR_LOAD_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "channel",
        kind: "i32 BGM channel",
        description: "Resident BGM slot 0..15.",
    },
    NativeParameterSpec {
        name: "archive",
        kind: "Shift-JIS archive/root",
        description: "Archive or resource root for both streams.",
    },
    NativeParameterSpec {
        name: "intro_file",
        kind: "Shift-JIS resource name",
        description: "Initial stream played before the loop stream.",
    },
    NativeParameterSpec {
        name: "loop_file",
        kind: "Shift-JIS resource name",
        description: "Loop-region stream.",
    },
    NativeParameterSpec {
        name: "loop_mode",
        kind: "i32/bool",
        description: "Non-zero enables the two-stream loop transition.",
    },
    NativeParameterSpec {
        name: "volume",
        kind: "i32 0..128",
        description: "Initial per-channel BGM volume.",
    },
    NativeParameterSpec {
        name: "pan",
        kind: "i32 0..128",
        description: "Native pan with center 64.",
    },
];

const SOUND_BGM_CONTROL_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "channel",
        kind: "i32 BGM channel",
        description: "Resident BGM slot 0..15.",
    },
    NativeParameterSpec {
        name: "action",
        kind: "i32",
        description: "Zero pauses; non-zero starts a newly loaded buffer or resumes it.",
    },
];

const SOUND_BGM_QUERY_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "channel",
        kind: "i32 BGM channel",
        description: "Resident BGM slot 0..15.",
    },
    NativeParameterSpec {
        name: "state_or_position_out",
        kind: "BP pointer to i32",
        description: "Receives the target buffer state/position field. The VM owns this write.",
    },
];

const SOUND_BGM_FADE_VOLUME_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "channel",
        kind: "i32 BGM channel",
        description: "Resident BGM slot 0..15.",
    },
    NativeParameterSpec {
        name: "volume",
        kind: "i32 0..128",
        description: "New per-channel BGM gain.",
    },
    NativeParameterSpec {
        name: "duration_ms",
        kind: "i32 milliseconds",
        description: "Transition duration.",
    },
];

const SOUND_CHANNEL_PAN_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "channel",
        kind: "i32 BGM channel",
        description: "Resident BGM slot 0..15.",
    },
    NativeParameterSpec {
        name: "pan",
        kind: "i32 0..128",
        description: "Native pan with center 64.",
    },
];

const SOUND_CHANNEL_DURATION_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "channel",
        kind: "i32 channel index",
        description: "BGM 0..15 or SE 0..63, depending on the selector.",
    },
    NativeParameterSpec {
        name: "duration_ms",
        kind: "i32 milliseconds",
        description: "Transition duration.",
    },
];

const SOUND_SE_LOAD_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "channel",
        kind: "i32 SE channel",
        description: "Resident SE slot 0..63.",
    },
    NativeParameterSpec {
        name: "archive",
        kind: "Shift-JIS archive/root",
        description: "Archive or resource root.",
    },
    NativeParameterSpec {
        name: "file",
        kind: "Shift-JIS resource name",
        description: "Sound-effect resource.",
    },
];

const SOUND_SE_LOAD_SCALED_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "channel",
        kind: "i32 SE channel",
        description: "Resident SE slot 0..63.",
    },
    NativeParameterSpec {
        name: "archive",
        kind: "Shift-JIS archive/root",
        description: "Archive or resource root.",
    },
    NativeParameterSpec {
        name: "file",
        kind: "Shift-JIS resource name",
        description: "Sound-effect resource.",
    },
    NativeParameterSpec {
        name: "native_start_parameter",
        kind: "i32",
        description: "Opaque value forwarded to the target sound-object start/configuration virtual call; it is not a fade duration.",
    },
    NativeParameterSpec {
        name: "decode_gain_16_16",
        kind: "signed 16.16 fixed point",
        description: "Decoder/sample gain scale.",
    },
];

const SOUND_SE_PLAY_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "channel",
        kind: "i32 SE channel",
        description: "Resident SE slot 0..63.",
    },
    NativeParameterSpec {
        name: "volume",
        kind: "i32 0..128",
        description: "Per-play SE volume.",
    },
    NativeParameterSpec {
        name: "pan",
        kind: "i32 0..128",
        description: "Native pan with center 64.",
    },
];

const SOUND_SE_LOAD_CUSTOM_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "channel",
        kind: "i32 SE channel",
        description: "Resident SE slot 0..63.",
    },
    NativeParameterSpec {
        name: "archive",
        kind: "Shift-JIS archive/root",
        description: "Archive or resource root.",
    },
    NativeParameterSpec {
        name: "file",
        kind: "Shift-JIS resource name",
        description: "Sound-effect resource.",
    },
    NativeParameterSpec {
        name: "native_start_parameter",
        kind: "i32",
        description: "Opaque value forwarded to the target sound-object start/configuration virtual call.",
    },
    NativeParameterSpec {
        name: "decode_gain_16_16",
        kind: "signed 16.16 fixed point",
        description: "Decoder/sample gain scale.",
    },
    NativeParameterSpec {
        name: "playback_rate_16_16",
        kind: "signed 16.16 fixed point",
        description: "Playback-rate scale.",
    },
];

const SOUND_SE_REGISTER_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "channel",
        kind: "i32 SE channel",
        description: "Resident SE slot 0..63.",
    },
    NativeParameterSpec {
        name: "source",
        kind: "native sound descriptor pointer/value",
        description: "Memory-backed sound registration source.",
    },
    NativeParameterSpec {
        name: "native_start_parameter",
        kind: "i32",
        description: "Opaque target sound-object start/configuration value.",
    },
    NativeParameterSpec {
        name: "decode_gain_16_16",
        kind: "signed 16.16 fixed point",
        description: "Decoder/sample gain scale.",
    },
    NativeParameterSpec {
        name: "playback_rate_16_16",
        kind: "signed 16.16 fixed point",
        description: "Playback-rate scale.",
    },
];

const SOUND_CHANNEL_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "channel",
    kind: "i32 channel index",
    description: "BGM 0..15 or SE 0..63, depending on the selector.",
}];

const SOUND_POSITION_QUERY_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "channel",
    kind: "i32 SE channel",
    description: "Resident SE slot 0..63.",
}];

const SOUND_CD_PLAY_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "track",
        kind: "i32",
        description: "MCI CD-audio track number.",
    },
    NativeParameterSpec {
        name: "option",
        kind: "i32",
        description: "Secondary target MCI play option.",
    },
];

const SOUND_CD_QUERY_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "mode_out",
    kind: "BP pointer to i32",
    description: "Receives the target MCI mode. The VM owns this write.",
}];

const SOUND_WAVE_PLAY_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "path",
    kind: "Shift-JIS path",
    description: "Wave resource passed to PlaySoundA with asynchronous nodefault filename flags.",
}];
// SystemB0 target-confirmed parameter contracts.
const USER_DRAW_BITMAP_TO_WINDOW_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "x",
        kind: "i32",
        description: "Destination x coordinate in the parent client area.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32",
        description: "Destination y coordinate in the parent client area.",
    },
    NativeParameterSpec {
        name: "resource",
        kind: "graph resource id",
        description: "Bitmap resource below 0x4000 resolved through the target graph resource manager.",
    },
];
const USER_SET_MAIN_WINDOW_POSITION_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "x",
        kind: "i32",
        description: "Requested outer-window x coordinate.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32",
        description: "Requested outer-window y coordinate.",
    },
];
const USER_BIND_CURSOR_OBJECT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "object",
        kind: "display object handle",
        description: "Zero removes the cursor binding; nonzero follows the physical cursor.",
    },
    NativeParameterSpec {
        name: "x_offset",
        kind: "i32",
        description: "Signed x offset from the physical cursor.",
    },
    NativeParameterSpec {
        name: "y_offset",
        kind: "i32",
        description: "Signed y offset from the physical cursor.",
    },
];
const USER_SET_CURSOR_IDLE_TIMEOUT_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "timeout_ms",
    kind: "i32 milliseconds",
    description: "Zero disables idle hiding; positive values arm the cursor-idle timer.",
}];
const USER_SHAKE_SCREEN_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "direction_mode",
        kind: "i32 enum 0..3",
        description: "Selects horizontal, vertical, paired, or opposing-axis motion.",
    },
    NativeParameterSpec {
        name: "amplitude",
        kind: "i32",
        description: "Initial target shake amplitude, stored internally as amplitude << 11.",
    },
    NativeParameterSpec {
        name: "oscillation_ticks",
        kind: "positive i32",
        description: "Native oscillation divisor.",
    },
    NativeParameterSpec {
        name: "cycle_count",
        kind: "positive i32",
        description: "Number of complete triangular shake cycles.",
    },
    NativeParameterSpec {
        name: "damping_percent",
        kind: "i32 percent",
        description: "Per-cycle attenuation percentage.",
    },
    NativeParameterSpec {
        name: "updates_per_second",
        kind: "positive i32",
        description: "Native update frequency; must be at least oscillation_ticks.",
    },
    NativeParameterSpec {
        name: "lock_renderer",
        kind: "BOOL",
        description: "Requests the target renderer lock while the procedure runs.",
    },
];
const USER_CREATE_DEBUG_WINDOW_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "title",
        kind: "Shift-JIS string",
        description: "Auxiliary window title.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "i32",
        description: "Initial outer-window x coordinate.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32",
        description: "Initial outer-window y coordinate.",
    },
    NativeParameterSpec {
        name: "width",
        kind: "i32 32..2048",
        description: "Client width.",
    },
    NativeParameterSpec {
        name: "height",
        kind: "i32 32..2048",
        description: "Client height.",
    },
];
const USER_CLOSE_DEBUG_WINDOW_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "handle",
    kind: "0xFF000000-tagged handle",
    description: "Auxiliary window to destroy.",
}];
const USER_SET_DEBUG_WINDOW_VISIBLE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "debug-window handle",
        description: "Target auxiliary window.",
    },
    NativeParameterSpec {
        name: "visible",
        kind: "BOOL",
        description: "Show or hide the auxiliary window.",
    },
];
const USER_SET_DEBUG_WINDOW_TITLE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "debug-window handle",
        description: "Target auxiliary window.",
    },
    NativeParameterSpec {
        name: "title",
        kind: "Shift-JIS string",
        description: "Replacement title.",
    },
];
const USER_MOVE_DEBUG_WINDOW_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "debug-window handle",
        description: "Target auxiliary window.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "i32",
        description: "New outer-window x coordinate.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32",
        description: "New outer-window y coordinate.",
    },
];
const USER_GET_DEBUG_WINDOW_POSITION_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "handle",
    kind: "debug-window handle",
    description: "Target auxiliary window.",
}];
const USER_CLEAR_DEBUG_WINDOW_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "debug-window handle",
        description: "Target auxiliary window.",
    },
    NativeParameterSpec {
        name: "color",
        kind: "packed RGB",
        description: "Backbuffer clear color.",
    },
];
const USER_DRAW_DEBUG_BITMAP_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "debug-window handle",
        description: "Target auxiliary window.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "i32",
        description: "Destination x coordinate.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32",
        description: "Destination y coordinate.",
    },
    NativeParameterSpec {
        name: "bitmap",
        kind: "graph resource id",
        description: "Source bitmap below 0x4000.",
    },
    NativeParameterSpec {
        name: "blend_mode",
        kind: "i32",
        description: "Target-supported bitmap blend selector.",
    },
    NativeParameterSpec {
        name: "alpha",
        kind: "i32 0..256",
        description: "Native alpha parameter.",
    },
];
const USER_DRAW_DEBUG_TEXT_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "debug-window handle",
        description: "Target auxiliary window.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "i32",
        description: "Text origin x.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32",
        description: "Text origin y.",
    },
    NativeParameterSpec {
        name: "text",
        kind: "Shift-JIS string",
        description: "Text to rasterize.",
    },
    NativeParameterSpec {
        name: "font_id",
        kind: "font registry id",
        description: "Target font record.",
    },
    NativeParameterSpec {
        name: "font_size",
        kind: "i32",
        description: "Font pixel size.",
    },
    NativeParameterSpec {
        name: "style",
        kind: "i32",
        description: "Native text style field.",
    },
    NativeParameterSpec {
        name: "color",
        kind: "packed RGB",
        description: "Text color.",
    },
    NativeParameterSpec {
        name: "blend_mode",
        kind: "i32",
        description: "Native text blend field.",
    },
    NativeParameterSpec {
        name: "alpha",
        kind: "i32",
        description: "Native text alpha field.",
    },
];
const USER_SET_DEBUG_WINDOW_CLOSE_MESSAGE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "debug-window handle",
        description: "Target auxiliary window.",
    },
    NativeParameterSpec {
        name: "message",
        kind: "Shift-JIS string",
        description: "Per-window close confirmation/message text.",
    },
];
const USER_CREATE_EDIT_CONTROL_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "x",
        kind: "i32",
        description: "Edit-control x coordinate.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32",
        description: "Edit-control y coordinate.",
    },
    NativeParameterSpec {
        name: "width",
        kind: "i32",
        description: "Edit-control width, at least 8.",
    },
    NativeParameterSpec {
        name: "height",
        kind: "i32",
        description: "Edit-control height, at least 8.",
    },
    NativeParameterSpec {
        name: "font_id",
        kind: "font registry id",
        description: "Font face selected through the target font registry.",
    },
    NativeParameterSpec {
        name: "font_size",
        kind: "i32 8..64",
        description: "Font pixel size.",
    },
    NativeParameterSpec {
        name: "max_chars",
        kind: "i32 1..256",
        description: "Maximum text length.",
    },
    NativeParameterSpec {
        name: "focus",
        kind: "BOOL",
        description: "Whether the control immediately receives focus.",
    },
];
const USER_SET_EDIT_FONT_SCALE_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "percent",
    kind: "i32 25..200",
    description: "Edit font scaling percentage.",
}];
const USER_SET_EDIT_VISIBLE_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "visible",
    kind: "BOOL",
    description: "Show or hide the edit control.",
}];
const USER_SET_EDIT_COLOR_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "color",
    kind: "packed RGB",
    description: "Color converted to the target internal byte order.",
}];
const USER_SET_EDIT_TEXT_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "text",
    kind: "Shift-JIS string",
    description: "Replacement edit-control text, bounded to the target 256-byte buffer.",
}];
const USER_GET_EDIT_TEXT_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "destination",
    kind: "BP pointer to 256-byte buffer",
    description: "Receives at most 255 Shift-JIS bytes plus NUL.",
}];
const USER_SET_EDIT_HIDE_ON_ENTER_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "enabled",
    kind: "BOOL",
    description: "When set, Enter returns focus and hides the control.",
}];
const USER_SET_EDIT_PRINTABLE_INPUT_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "enabled",
    kind: "BOOL",
    description: "Allows or suppresses printable characters while preserving Tab/Enter handling.",
}];
const USER_SHOW_MESSAGE_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "message",
    kind: "Shift-JIS string",
    description: "Informational message text.",
}];
const USER_SHOW_YES_NO_MESSAGE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "message",
        kind: "Shift-JIS string",
        description: "Question text.",
    },
    NativeParameterSpec {
        name: "topmost",
        kind: "BOOL",
        description: "Controls the target task-modal flag.",
    },
];
const USER_SHOW_TYPED_MESSAGE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "message",
        kind: "Shift-JIS string",
        description: "Message text.",
    },
    NativeParameterSpec {
        name: "kind",
        kind: "i32",
        description: "One selects OK/cancel-style success on IDOK; other values select yes/no success on IDYES.",
    },
    NativeParameterSpec {
        name: "topmost",
        kind: "BOOL",
        description: "Controls the target task-modal flag.",
    },
];
const USER_SET_MESSAGE_TITLE_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "title",
    kind: "Shift-JIS string",
    description: "Global message-box title.",
}];
const USER_SHOW_INPUT_DIALOG_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "buffer",
        kind: "BP pointer to mutable Shift-JIS buffer",
        description: "Receives at most abs(max_length), capped to the target 255-byte payload.",
    },
    NativeParameterSpec {
        name: "title",
        kind: "Shift-JIS string",
        description: "Dialog window title.",
    },
    NativeParameterSpec {
        name: "initial",
        kind: "Shift-JIS string",
        description: "Initial edit-control contents.",
    },
    NativeParameterSpec {
        name: "max_length",
        kind: "signed i32",
        description: "Absolute value is the byte limit; a negative value enables signed-decimal filtering.",
    },
];
const USER_SHOW_MULTI_FIELD_DIALOG_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "first_out",
        kind: "BP pointer to mutable Shift-JIS buffer",
        description: "Receives the first field, at most 255 bytes plus NUL.",
    },
    NativeParameterSpec {
        name: "second_out",
        kind: "BP pointer to mutable Shift-JIS buffer",
        description: "Receives the second field, at most 255 bytes plus NUL.",
    },
    NativeParameterSpec {
        name: "title",
        kind: "Shift-JIS string",
        description: "Dialog window title.",
    },
    NativeParameterSpec {
        name: "first_label",
        kind: "Shift-JIS string",
        description: "Label for the first edit field.",
    },
    NativeParameterSpec {
        name: "first_initial",
        kind: "Shift-JIS string",
        description: "Initial first-field contents.",
    },
    NativeParameterSpec {
        name: "first_max_length",
        kind: "i32",
        description: "First field byte limit, capped to 255.",
    },
    NativeParameterSpec {
        name: "second_label",
        kind: "Shift-JIS string",
        description: "Label for the second edit field.",
    },
    NativeParameterSpec {
        name: "second_initial",
        kind: "Shift-JIS string",
        description: "Initial second-field contents.",
    },
    NativeParameterSpec {
        name: "second_max_length",
        kind: "i32",
        description: "Second field byte limit, capped to 255.",
    },
];
const USER_SHOW_SELECTION_DIALOG_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "buffer",
        kind: "BP pointer to mutable Shift-JIS buffer",
        description: "Receives four edited segments joined by '-'.",
    },
    NativeParameterSpec {
        name: "title",
        kind: "Shift-JIS string",
        description: "Dialog window title.",
    },
    NativeParameterSpec {
        name: "prompt",
        kind: "Shift-JIS string",
        description: "Prompt shown above the four segment fields.",
    },
    NativeParameterSpec {
        name: "segment_max_length",
        kind: "signed i32",
        description: "Absolute per-segment byte limit, capped to 32; negative enables signed-decimal filtering.",
    },
    NativeParameterSpec {
        name: "extra",
        kind: "i32",
        description: "Target positional option stored by the native dialog.",
    },
];
const USER_SHOW_EXTENDED_DIALOG_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "template",
        kind: "i32",
        description: "Selects the target extended two-field dialog template.",
    },
    NativeParameterSpec {
        name: "first_out",
        kind: "BP pointer to mutable Shift-JIS buffer",
        description: "Receives the first field.",
    },
    NativeParameterSpec {
        name: "second_out",
        kind: "BP pointer to mutable Shift-JIS buffer",
        description: "Receives the second field.",
    },
    NativeParameterSpec {
        name: "title",
        kind: "Shift-JIS string",
        description: "Dialog window title.",
    },
    NativeParameterSpec {
        name: "first_label",
        kind: "Shift-JIS string",
        description: "Label for the first field.",
    },
    NativeParameterSpec {
        name: "first_initial",
        kind: "Shift-JIS string",
        description: "Initial first-field contents.",
    },
    NativeParameterSpec {
        name: "first_max_length",
        kind: "i32",
        description: "First field byte limit, capped to 255.",
    },
    NativeParameterSpec {
        name: "first_numeric",
        kind: "BOOL",
        description: "Enables target signed-decimal filtering for the first field.",
    },
    NativeParameterSpec {
        name: "second_label",
        kind: "Shift-JIS string",
        description: "Label for the second field.",
    },
    NativeParameterSpec {
        name: "second_initial",
        kind: "Shift-JIS string",
        description: "Initial second-field contents.",
    },
    NativeParameterSpec {
        name: "second_max_length",
        kind: "i32",
        description: "Second field byte limit, capped to 255.",
    },
    NativeParameterSpec {
        name: "second_numeric",
        kind: "BOOL",
        description: "Enables target signed-decimal filtering for the second field.",
    },
];
const USER_SHOW_PATH_DIALOG_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "buffer",
        kind: "BP pointer to mutable Shift-JIS buffer",
        description: "Receives the selected list row.",
    },
    NativeParameterSpec {
        name: "title",
        kind: "Shift-JIS string",
        description: "Dialog window title.",
    },
    NativeParameterSpec {
        name: "prompt",
        kind: "Shift-JIS string",
        description: "List prompt text.",
    },
    NativeParameterSpec {
        name: "newline_options",
        kind: "Shift-JIS string",
        description: "Newline-delimited list entries.",
    },
];
const USER_SHOW_SIX_FIELD_DIALOG_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "field1",
        kind: "BP pointer to mutable 11-byte buffer",
        description: "First text field, limited to 10 payload bytes.",
    },
    NativeParameterSpec {
        name: "field2",
        kind: "BP pointer to mutable 11-byte buffer",
        description: "Second text field, limited to 10 payload bytes.",
    },
    NativeParameterSpec {
        name: "field3",
        kind: "BP pointer to mutable 11-byte buffer",
        description: "Third text field, limited to 10 payload bytes.",
    },
    NativeParameterSpec {
        name: "field4",
        kind: "BP pointer to mutable 11-byte buffer",
        description: "Fourth text field, limited to 10 payload bytes.",
    },
    NativeParameterSpec {
        name: "month_index",
        kind: "BP pointer to i32",
        description: "Zero-based month selection, updated on acceptance.",
    },
    NativeParameterSpec {
        name: "day_index",
        kind: "BP pointer to i32",
        description: "Zero-based day selection, updated on acceptance.",
    },
];
const USER_CREATE_MODELESS_DIALOG_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle_out",
        kind: "BP pointer to i32",
        description: "Receives the allocated modeless-dialog handle.",
    },
    NativeParameterSpec {
        name: "mode",
        kind: "i32",
        description: "Only zero creates the target dialog.",
    },
    NativeParameterSpec {
        name: "initial_state",
        kind: "BP pointer to nine i32 values",
        description: "Five slider values followed by four radio-group selections.",
    },
];
const USER_CLOSE_MODELESS_DIALOG_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "handle",
    kind: "modeless-dialog handle",
    description: "Dialog to close and release.",
}];
const USER_SET_MODELESS_DIALOG_VISIBLE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "modeless-dialog handle",
        description: "Target dialog.",
    },
    NativeParameterSpec {
        name: "visible",
        kind: "BOOL",
        description: "Show or hide the modeless dialog.",
    },
];
const USER_POLL_MODELESS_DIALOG_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "event_out",
        kind: "BP pointer to two i32 values",
        description: "Receives the next two-DWORD event when available.",
    },
    NativeParameterSpec {
        name: "handle",
        kind: "modeless-dialog handle",
        description: "Dialog event source.",
    },
];
const USER_INTERN_FONT_NAME_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "name",
    kind: "Shift-JIS font name",
    description: "Name interned with option -1.",
}];
const USER_INTERN_FONT_NAME_WITH_OPTION_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "name",
        kind: "Shift-JIS font name",
        description: "Font name to intern.",
    },
    NativeParameterSpec {
        name: "option",
        kind: "i32",
        description: "Target charset/metadata option forwarded to sub_468A70.",
    },
];
const USER_REGISTER_FONT_RESOURCE_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "resource",
    kind: "Shift-JIS path/resource name",
    description: "Font file registered through AddFontResourceA on the target.",
}];
const USER_REGISTER_ARCHIVE_FONT_RESOURCE_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "resource",
        kind: "Shift-JIS font resource name",
        description: "Font payload name.",
    },
    NativeParameterSpec {
        name: "archive",
        kind: "Shift-JIS archive/source name",
        description: "Archive or memory source forwarded to sub_468BD0.",
    },
];
const USER_QUERY_FONT_AVAILABLE_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "name",
    kind: "Shift-JIS font family",
    description: "Family enumerated through the target Shift-JIS font callback.",
}];
const USER_QUERY_FONT_CAPABILITY_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "first",
        kind: "Shift-JIS font name",
        description: "First font/capability key.",
    },
    NativeParameterSpec {
        name: "second",
        kind: "Shift-JIS font name",
        description: "Second font/capability key.",
    },
];
const USER_SET_FONT_ALIAS_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "alias",
        kind: "Shift-JIS font alias",
        description: "Alias/default-font key.",
    },
    NativeParameterSpec {
        name: "face",
        kind: "Shift-JIS font face",
        description: "Resolved host face name.",
    },
];
const USER_SET_DESKTOP_WALLPAPER_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "path",
        kind: "Shift-JIS path",
        description: "Wallpaper image path.",
    },
    NativeParameterSpec {
        name: "style",
        kind: "i32",
        description: "Target wallpaper style value.",
    },
    NativeParameterSpec {
        name: "tile",
        kind: "i32",
        description: "Target wallpaper tile value.",
    },
];

// SystemC0 target-confirmed parameter contracts.
const USER_C0_00_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "width",
        kind: "i32",
        description: "Requested particle-screen width.",
    },
    NativeParameterSpec {
        name: "height",
        kind: "i32",
        description: "Requested particle-screen height.",
    },
];
const USER_C0_01_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "handle",
    kind: "0xC0000000-tagged handle",
    description: "Particle screen to detach and release.",
}];
const USER_C0_04_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "particle-screen handle",
        description: "Target particle screen.",
    },
    NativeParameterSpec {
        name: "enabled",
        kind: "BOOL",
        description: "Display-object enable state.",
    },
];
const USER_C0_05_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "handle", kind: "particle-screen handle", description: "Target particle screen." },
    NativeParameterSpec { name: "x", kind: "i32", description: "Display X." },
    NativeParameterSpec { name: "y", kind: "i32", description: "Display Y." },
    NativeParameterSpec { name: "blend_mode", kind: "native blend enum", description: "Composition selector." },
    NativeParameterSpec { name: "alpha", kind: "u32 <=256", description: "Inverse transparency." },
    NativeParameterSpec { name: "priority", kind: "u32 <65536", description: "Display-chain priority." },
];
const USER_C0_06_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "particle-screen handle",
        description: "Target particle screen.",
    },
    NativeParameterSpec {
        name: "count",
        kind: "u32",
        description: "Number of frame-table entries.",
    },
    NativeParameterSpec {
        name: "first_table",
        kind: "BP pointer to i32[count]",
        description: "Target duration/weight table; a null pointer with count one selects 0x7FFF.",
    },
    NativeParameterSpec {
        name: "second_table",
        kind: "BP pointer to i32[count]",
        description: "Target companion frame table; the one-entry default is zero.",
    },
];
const USER_C0_08_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "handle",
    kind: "particle-screen handle",
    description: "Target particle screen.",
}];
const USER_C0_09_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "particle-screen handle",
        description: "Target particle screen.",
    },
    NativeParameterSpec {
        name: "interval",
        kind: "i32 native ticks/ms",
        description: "Zero removes the object from the automatic update list; nonzero installs/updates it.",
    },
];
const USER_C0_0A_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "handle", kind: "particle-screen handle", description: "Target particle screen." },
    NativeParameterSpec { name: "rectangle_threshold", kind: "i32", description: "Dirty-rectangle count at which native redraws the whole object; portable rendering redraws the whole frame." },
];
const USER_C0_0B_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "handle", kind: "particle-screen handle", description: "Target particle screen." },
    NativeParameterSpec { name: "camera_x", kind: "i32", description: "Camera translation in source 16.16 coordinates; converted to 24.8 internally." },
    NativeParameterSpec { name: "camera_y", kind: "i32", description: "Camera translation in source 16.16 coordinates; converted to 24.8 internally." },
    NativeParameterSpec { name: "camera_z", kind: "i32", description: "Camera translation in source 16.16 coordinates; converted to 24.8 internally." },
    NativeParameterSpec { name: "angle_x", kind: "16.16 degrees", description: "Euler rotation, applied X then Y then Z." },
    NativeParameterSpec { name: "angle_y", kind: "16.16 degrees", description: "Euler rotation, applied X then Y then Z." },
    NativeParameterSpec { name: "angle_z", kind: "16.16 degrees", description: "Euler rotation, applied X then Y then Z." },
    NativeParameterSpec { name: "focal_distance", kind: "i32 >0", description: "Perspective projection distance." },
    NativeParameterSpec { name: "center_x", kind: "i32", description: "Projection origin X." },
    NativeParameterSpec { name: "center_y", kind: "i32", description: "Projection origin Y." },
];
const USER_C0_0C_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "handle", kind: "particle-screen handle", description: "Target particle screen." },
    NativeParameterSpec { name: "interval_ms", kind: "u32 <=100", description: "Simulation tick interval; zero suspends simulation. Recomputes wind segment durations." },
];
const USER_C0_0D_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "particle-screen handle",
        description: "Target particle screen.",
    },
    NativeParameterSpec {
        name: "step",
        kind: "i32",
        description: "Native particle simulation advance value.",
    },
];
const USER_C0_0F_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "handle",
    kind: "particle-screen handle",
    description: "Target particle screen.",
}];
const USER_C0_10_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "handle", kind: "particle-screen handle", description: "Target particle screen." },
    NativeParameterSpec { name: "enabled", kind: "BOOL", description: "Enables the environment air field." },
    NativeParameterSpec { name: "wind_x", kind: "i32", description: "Wind vector bank, converted by arithmetic right shift eight." },
    NativeParameterSpec { name: "spread_x", kind: "i32", description: "Wind vector bank, converted by arithmetic right shift eight." },
    NativeParameterSpec { name: "wind_y", kind: "i32", description: "Wind vector bank, converted by arithmetic right shift eight." },
    NativeParameterSpec { name: "spread_y", kind: "i32", description: "Wind vector bank, converted by arithmetic right shift eight." },
    NativeParameterSpec { name: "wind_z", kind: "i32", description: "Wind vector bank, converted by arithmetic right shift eight." },
    NativeParameterSpec { name: "spread_z", kind: "i32", description: "Wind vector bank, converted by arithmetic right shift eight." },
    NativeParameterSpec { name: "hold_duration", kind: "u32 ms", description: "Wind segment timing divided by the simulation interval." },
    NativeParameterSpec { name: "hold_spread", kind: "u32 ms", description: "Wind segment timing divided by the simulation interval." },
    NativeParameterSpec { name: "transition_duration", kind: "u32 ms", description: "Wind segment timing divided by the simulation interval." },
    NativeParameterSpec { name: "transition_spread", kind: "u32 ms", description: "Wind segment timing divided by the simulation interval." },
];
const USER_C0_18_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "kind", kind: "enum 0..1", description: "Zero is snow, one is firefly." },
    NativeParameterSpec { name: "bank", kind: "u32 <64", description: "Particle animation/style bank." },
    NativeParameterSpec { name: "frame_count", kind: "u32 <=32", description: "Copied animation frames." },
    NativeParameterSpec { name: "first_bitmap", kind: "bitmap handle or -1", description: "Consecutive RGB/ARGB bitmaps of equal dimensions and format." },
    NativeParameterSpec { name: "period", kind: "u32", description: "Base animation timing divisor." },
    NativeParameterSpec { name: "period_spread", kind: "u32", description: "Timing spread, interpreted by the interpolation mode." },
    NativeParameterSpec { name: "numerator", kind: "u32", description: "Numerator of the 16.16 animation step." },
];
const USER_C0_1A_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "kind", kind: "enum 0..1", description: "Zero is snow, one is firefly." },
    NativeParameterSpec { name: "bank", kind: "u32 <64", description: "Particle animation/style bank." },
    NativeParameterSpec { name: "frame_count", kind: "u32 <=32", description: "Copied animation frames." },
    NativeParameterSpec { name: "first_bitmap", kind: "bitmap handle or -1", description: "Consecutive RGB/ARGB bitmaps of equal dimensions and format." },
    NativeParameterSpec { name: "period", kind: "u32", description: "Base animation timing divisor." },
    NativeParameterSpec { name: "period_spread", kind: "u32", description: "Timing spread, interpreted by the interpolation mode." },
    NativeParameterSpec { name: "numerator", kind: "u32", description: "Numerator of the 16.16 animation step." },
    NativeParameterSpec { name: "mix", kind: "i32", description: "Secondary firefly animation mixing factor; kind must be one." },
];
const USER_C0_1B_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "kind", kind: "enum 0..1", description: "Zero is snow, one is firefly." },
    NativeParameterSpec { name: "bank", kind: "u32 <64", description: "Particle animation/style bank." },
    NativeParameterSpec { name: "mix", kind: "i32", description: "Mixing factor of an already-defined secondary firefly bank." },
];
const USER_C0_1F_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "mode",
    kind: "enum 0..2",
    description: "Global target particle interpolation mode.",
}];
const USER_C0_20_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "handle", kind: "particle-screen handle", description: "Target particle screen." },
    NativeParameterSpec { name: "bank", kind: "u32 <64", description: "Particle animation/style bank." },
    NativeParameterSpec { name: "count", kind: "u32", description: "Requested live particles; aggregate across both kinds must not exceed 32768." },
    NativeParameterSpec { name: "spacing_ms", kind: "u32", description: "Time between emissions." },
];
const USER_C0_24_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "bank", kind: "u32 <64", description: "Particle animation/style bank." },
    NativeParameterSpec { name: "bitmap", kind: "bitmap handle or -1", description: "Single-frame primary animation definition." },
];
const USER_C0_25_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "bank", kind: "u32 <64", description: "Particle animation/style bank." },
    NativeParameterSpec { name: "x_spread", kind: "i32", description: "Snow distribution/motion field converted by arithmetic right shift eight; spreads use absolute values." },
    NativeParameterSpec { name: "start_y", kind: "i32", description: "Snow distribution/motion field converted by arithmetic right shift eight; spreads use absolute values." },
    NativeParameterSpec { name: "z_spread", kind: "i32", description: "Snow distribution/motion field converted by arithmetic right shift eight; spreads use absolute values." },
    NativeParameterSpec { name: "velocity_x", kind: "i32", description: "Snow distribution/motion field converted by arithmetic right shift eight; spreads use absolute values." },
    NativeParameterSpec { name: "velocity_x_spread", kind: "i32", description: "Snow distribution/motion field converted by arithmetic right shift eight; spreads use absolute values." },
    NativeParameterSpec { name: "velocity_y", kind: "i32", description: "Snow distribution/motion field converted by arithmetic right shift eight; spreads use absolute values." },
    NativeParameterSpec { name: "velocity_y_spread", kind: "i32", description: "Snow distribution/motion field converted by arithmetic right shift eight; spreads use absolute values." },
    NativeParameterSpec { name: "velocity_z", kind: "i32", description: "Snow distribution/motion field converted by arithmetic right shift eight; spreads use absolute values." },
    NativeParameterSpec { name: "velocity_z_spread", kind: "i32", description: "Snow distribution/motion field converted by arithmetic right shift eight; spreads use absolute values." },
    NativeParameterSpec { name: "wind_coefficient", kind: "BP pointer to i32", description: "Dereferenced DWORD converted by right shift eight." },
];
const USER_C0_28_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "handle", kind: "particle-screen handle", description: "Target particle screen." },
    NativeParameterSpec { name: "bank", kind: "u32 <64", description: "Particle animation/style bank." },
    NativeParameterSpec { name: "count", kind: "u32", description: "Requested live particles; aggregate across both kinds must not exceed 32768." },
    NativeParameterSpec { name: "spacing_ms", kind: "u32", description: "Time between emissions." },
];
const USER_C0_29_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "handle", kind: "particle-screen handle", description: "Target particle screen." },
    NativeParameterSpec { name: "bank", kind: "u32 <64", description: "Particle animation/style bank." },
    NativeParameterSpec { name: "origin_x", kind: "i32", description: "Recovered per-screen firefly emitter field; see particle.c for native conversion." },
    NativeParameterSpec { name: "origin_y_spread", kind: "i32", description: "Recovered per-screen firefly emitter field; see particle.c for native conversion." },
    NativeParameterSpec { name: "origin_z", kind: "i32", description: "Recovered per-screen firefly emitter field; see particle.c for native conversion." },
    NativeParameterSpec { name: "normalize_velocity", kind: "i32", description: "Recovered per-screen firefly emitter field; see particle.c for native conversion." },
    NativeParameterSpec { name: "speed", kind: "i32", description: "Recovered per-screen firefly emitter field; see particle.c for native conversion." },
    NativeParameterSpec { name: "speed_spread", kind: "i32", description: "Recovered per-screen firefly emitter field; see particle.c for native conversion." },
    NativeParameterSpec { name: "axis_x", kind: "i32", description: "Recovered per-screen firefly emitter field; see particle.c for native conversion." },
    NativeParameterSpec { name: "axis_y", kind: "i32", description: "Recovered per-screen firefly emitter field; see particle.c for native conversion." },
    NativeParameterSpec { name: "axis_z", kind: "i32", description: "Recovered per-screen firefly emitter field; see particle.c for native conversion." },
    NativeParameterSpec { name: "adjust_lifetime", kind: "i32", description: "Recovered per-screen firefly emitter field; see particle.c for native conversion." },
    NativeParameterSpec { name: "interpolate_velocity", kind: "i32", description: "Recovered per-screen firefly emitter field; see particle.c for native conversion." },
    NativeParameterSpec { name: "rotate_velocity", kind: "i32", description: "Recovered per-screen firefly emitter field; see particle.c for native conversion." },
    NativeParameterSpec { name: "angle", kind: "i32", description: "Recovered per-screen firefly emitter field; see particle.c for native conversion." },
    NativeParameterSpec { name: "angle_spread", kind: "i32", description: "Recovered per-screen firefly emitter field; see particle.c for native conversion." },
];
const USER_C0_2C_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "bank", kind: "u32 <64", description: "Particle animation/style bank." },
    NativeParameterSpec { name: "bitmap", kind: "bitmap handle or -1", description: "Single-frame primary animation definition." },
];
const USER_C0_2D_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec { name: "bank", kind: "u32 <64", description: "Particle animation/style bank." },
    NativeParameterSpec { name: "lifetime", kind: "i32", description: "Firefly style field; see particle.c for absolute-value and fixed-point conversions." },
    NativeParameterSpec { name: "lifetime_spread", kind: "i32", description: "Firefly style field; see particle.c for absolute-value and fixed-point conversions." },
    NativeParameterSpec { name: "x_spread", kind: "i32", description: "Firefly style field; see particle.c for absolute-value and fixed-point conversions." },
    NativeParameterSpec { name: "start_y", kind: "i32", description: "Firefly style field; see particle.c for absolute-value and fixed-point conversions." },
    NativeParameterSpec { name: "z_spread", kind: "i32", description: "Firefly style field; see particle.c for absolute-value and fixed-point conversions." },
    NativeParameterSpec { name: "velocity_x", kind: "i32", description: "Firefly style field; see particle.c for absolute-value and fixed-point conversions." },
    NativeParameterSpec { name: "velocity_x_spread", kind: "i32", description: "Firefly style field; see particle.c for absolute-value and fixed-point conversions." },
    NativeParameterSpec { name: "velocity_y", kind: "i32", description: "Firefly style field; see particle.c for absolute-value and fixed-point conversions." },
    NativeParameterSpec { name: "velocity_y_spread", kind: "i32", description: "Firefly style field; see particle.c for absolute-value and fixed-point conversions." },
    NativeParameterSpec { name: "velocity_z", kind: "i32", description: "Firefly style field; see particle.c for absolute-value and fixed-point conversions." },
    NativeParameterSpec { name: "velocity_z_spread", kind: "i32", description: "Firefly style field; see particle.c for absolute-value and fixed-point conversions." },
    NativeParameterSpec { name: "segment_duration", kind: "i32", description: "Firefly style field; see particle.c for absolute-value and fixed-point conversions." },
    NativeParameterSpec { name: "segment_spread", kind: "i32", description: "Firefly style field; see particle.c for absolute-value and fixed-point conversions." },
    NativeParameterSpec { name: "wind_coefficient", kind: "BP pointer to i32", description: "Dereferenced DWORD, shifted right eight." },
    NativeParameterSpec { name: "fade_in", kind: "i32", description: "Zero becomes one." },
    NativeParameterSpec { name: "fade_out", kind: "i32", description: "Zero becomes one." },
    NativeParameterSpec { name: "blend_mode", kind: "native blend enum", description: "Per-particle composition selector." },
];
const USER_C0_40_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "width",
        kind: "i32",
        description: "Requested rain-screen width.",
    },
    NativeParameterSpec {
        name: "height",
        kind: "i32",
        description: "Requested rain-screen height.",
    },
];
const USER_C0_41_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "handle",
    kind: "0xC1000000-tagged handle",
    description: "Rain screen to detach and release.",
}];
const USER_C0_42_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "rain-screen handle",
        description: "Target rain screen.",
    },
    NativeParameterSpec {
        name: "count",
        kind: "i32",
        description: "Native rain generator initialization/count value.",
    },
];
const USER_C0_43_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "rain-screen handle",
        description: "Target rain screen.",
    },
    NativeParameterSpec {
        name: "resource",
        kind: "graph resource id or -1",
        description: "Rain drop texture resource.",
    },
];
const USER_C0_44_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "rain-screen handle",
        description: "Target rain screen.",
    },
    NativeParameterSpec {
        name: "enabled",
        kind: "BOOL",
        description: "Display-object enable state.",
    },
];
const USER_C0_45_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "rain-screen handle",
        description: "Target rain screen.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "i32",
        description: "Display x coordinate.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32",
        description: "Display y coordinate.",
    },
    NativeParameterSpec {
        name: "z",
        kind: "i32",
        description: "Display-chain depth.",
    },
    NativeParameterSpec {
        name: "blend_mode",
        kind: "native blend enum",
        description: "Validated target blend mode.",
    },
    NativeParameterSpec {
        name: "alpha",
        kind: "i32 0..256",
        description: "Target inverse-alpha parameter.",
    },
];
const USER_C0_46_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "rain-screen handle",
        description: "Target rain screen.",
    },
    NativeParameterSpec {
        name: "min_x",
        kind: "i32",
        description: "Minimum x volume bound.",
    },
    NativeParameterSpec {
        name: "min_y",
        kind: "i32",
        description: "Minimum y volume bound.",
    },
    NativeParameterSpec {
        name: "min_z",
        kind: "i32",
        description: "Minimum z volume bound.",
    },
    NativeParameterSpec {
        name: "max_x",
        kind: "i32",
        description: "Maximum x volume bound.",
    },
    NativeParameterSpec {
        name: "max_y",
        kind: "i32",
        description: "Maximum y volume bound.",
    },
    NativeParameterSpec {
        name: "max_z",
        kind: "i32",
        description: "Maximum z volume bound.",
    },
];
const USER_C0_47_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "rain-screen handle",
        description: "Target rain screen.",
    },
    NativeParameterSpec {
        name: "width",
        kind: "nonzero i32",
        description: "Drop width stored as 8.8 fixed point.",
    },
];
const USER_C0_48_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "rain-screen handle",
        description: "Target rain screen.",
    },
    NativeParameterSpec {
        name: "height",
        kind: "nonzero i32",
        description: "Drop height stored as 8.8 fixed point.",
    },
];
const USER_C0_49_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "rain-screen handle",
        description: "Target rain screen.",
    },
    NativeParameterSpec {
        name: "color",
        kind: "packed target color",
        description: "Rain drop color field.",
    },
];
const USER_C0_4A_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "rain-screen handle",
        description: "Target rain screen.",
    },
    NativeParameterSpec {
        name: "density",
        kind: "i32",
        description: "Rain density/count field.",
    },
];
const USER_C0_4B_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "rain-screen handle",
        description: "Target rain screen.",
    },
    NativeParameterSpec {
        name: "speed",
        kind: "nonzero i32",
        description: "Rain speed field.",
    },
];
const USER_C0_4C_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "rain-screen handle",
        description: "Target rain screen.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "i32",
        description: "Origin x.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32",
        description: "Origin y.",
    },
    NativeParameterSpec {
        name: "z",
        kind: "i32",
        description: "Origin z.",
    },
];
const USER_C0_4D_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "rain-screen handle",
        description: "Target rain screen.",
    },
    NativeParameterSpec {
        name: "x",
        kind: "i32",
        description: "Direction/angle x.",
    },
    NativeParameterSpec {
        name: "y",
        kind: "i32",
        description: "Direction/angle y.",
    },
    NativeParameterSpec {
        name: "z",
        kind: "i32",
        description: "Direction/angle z.",
    },
];
const USER_C0_4E_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "rain-screen handle",
        description: "Target rain screen.",
    },
    NativeParameterSpec {
        name: "length",
        kind: "nonzero i32",
        description: "Rain drop/travel length field.",
    },
];
const USER_C0_4F_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "native_step",
        kind: "i32",
        description: "Value forwarded to sub_424F60.",
    },
    NativeParameterSpec {
        name: "frequency",
        kind: "i32 1..1000",
        description: "Global rain update frequency; interval becomes 1000/frequency.",
    },
];
const USER_C0_C1_PARAMETERS: &[NativeParameterSpec] = &[NativeParameterSpec {
    name: "handle",
    kind: "spline handle",
    description: "Spline registry entry to release.",
}];
const USER_C0_C2_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "handle",
        kind: "spline handle",
        description: "Target CSpline entry.",
    },
    NativeParameterSpec {
        name: "duration",
        kind: "i32 >= 2",
        description: "Maximum valid sampling bound.",
    },
    NativeParameterSpec {
        name: "points",
        kind: "BP pointer to 16-byte records",
        description: "Each record contains x, y, z, and one padding DWORD.",
    },
    NativeParameterSpec {
        name: "count",
        kind: "i32 >= 2",
        description: "Number of point records.",
    },
];
const USER_C0_C3_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "output",
        kind: "BP pointer to three i32 values",
        description: "Receives interpolated x, y, z.",
    },
    NativeParameterSpec {
        name: "handle",
        kind: "spline handle",
        description: "Target CSpline entry.",
    },
    NativeParameterSpec {
        name: "time",
        kind: "i32",
        description: "Sample time in [0,duration).",
    },
];
const USER_C0_F0_PARAMETERS: &[NativeParameterSpec] = &[
    NativeParameterSpec {
        name: "archive",
        kind: "Shift-JIS archive/source",
        description: "Resource archive name.",
    },
    NativeParameterSpec {
        name: "table_out",
        kind: "BP pointer to pairs",
        description: "Receives count pairs {base + offset, stride}.",
    },
    NativeParameterSpec {
        name: "count_out",
        kind: "BP pointer to i32",
        description: "Receives entry count.",
    },
    NativeParameterSpec {
        name: "resource",
        kind: "Shift-JIS resource",
        description: "BWEF resource name.",
    },
    NativeParameterSpec {
        name: "base",
        kind: "i32",
        description: "Base added to each file offset.",
    },
];

const DOCUMENTED_OPCODES: &[NativeOpcodeSpec] = &[
    // System81 extended dispatcher.
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_SET_CLOCK_JUMP_THRESHOLD,
        symbol: "Sys81_04_SetClockJumpThreshold",
        parameters: SYS81_04_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_48BE20 -> sub_499390: 50..=60000 sets dword_503DE8 (initially 500), the jump sub_499290 treats as a pause.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_COPY_COORDINATE_SLOT,
        symbol: "Sys81_07_CopyCoordinateSlot",
        parameters: SYS81_07_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48BE50 -> sub_48F090: index < 5 copies dword_503E40[index], the logical point of the last press of left/right/middle/X1/X2 (WndProc sub_48F060), (-1, -1) before any.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_GET_USER_NAME,
        symbol: "Sys81_08_GetUserName",
        parameters: SYS81_08_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48BE80: GetUserNameA into a 257-byte buffer; a longer name leaves it unchanged.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_GET_COMPUTER_NAME,
        symbol: "Sys81_09_GetComputerName",
        parameters: SYS81_09_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48BEB0: GetComputerNameA with nSize 16 (NetBIOS name: host name's first label, upper case, 15 chars).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_GET_CPU_BRAND,
        symbol: "Sys81_0A_GetCpuBrand",
        parameters: SYS81_0A_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48BEE0 -> sub_46FC80: CPUID brand string through sub_496650 space folding; 0 without leaf 0x80000004 (no CPUID host).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_GET_CPU_DISPLAY_INFO,
        symbol: "Sys81_0B_GetCpuDisplayInfo",
        parameters: SYS81_0B_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48BF10 -> sub_45E9F0: D3DADAPTER_IDENTIFIER9 Description (renderer adapter name) through sub_496650 and the DriverVersion words; first argument ignored.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_GET_OS_VERSION,
        symbol: "Sys81_0C_GetOsVersion",
        parameters: SYS81_0C_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48BF40 -> sub_46FD60: GetVersionExA major/minor/build/platform and szCSDVersion; zeroed structure without Windows.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_GET_PHYSICAL_MEMORY_MB,
        symbol: "Sys81_0D_GetPhysicalMemoryMb",
        parameters: SYS81_0D_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48BFD0: GlobalMemoryStatusEx total/available physical memory >> 20.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_GET_ADJUSTED_DESKTOP_DIMENSIONS,
        symbol: "Sys81_0E_GetAdjustedDesktopDimensions",
        parameters: SYS81_0E_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48C060 -> sub_45EAE0(.., 1): start-up desktop mode, halved for side-by-side (>= 2.5:1) or stacked (<= 1.25:1) monitors, unsigned.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_IS_MAIN_WINDOW_MINIMIZED,
        symbol: "Sys81_0F_IsMainWindowMinimized",
        parameters: NO_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_48C080 pushes IsIconic(hWndParent); the window frontend refreshes the iconic state every frame and on focus changes.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_SWAP_INPUT_BINDING,
        symbol: "Sys81_10_SwapInputBinding",
        parameters: SYS81_10_PARAMETERS,
        returns: "previous value",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48C0A0 -> sub_46DFA0: swaps the per-key repeat flag of a virtual key; >= 0x100 is fatal (不正な仮想キーコード).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_COPY_KEYBOARD_STATE,
        symbol: "Sys81_11_CopyKeyboardState",
        parameters: SYS81_11_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48C0E0: GetKeyboardState (0x80 held, bit 0 toggled per press).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_SET_KEYBOARD_POLLING_OVERRIDE,
        symbol: "Sys81_14_SetKeyboardPollingOverride",
        parameters: SYS81_14_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48C100 -> sub_46DAA0: dword_566800 background key polling.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_CONFIGURE_POINTER_HISTORY,
        symbol: "Sys81_16_ConfigurePointerHistory",
        parameters: SYS81_16_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48C120 -> sub_46B720: touch history capacity <= 0x200 and minimum step; empties the history.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_COPY_POINTER_HISTORY,
        symbol: "Sys81_17_CopyPointerHistory",
        parameters: SYS81_17_PARAMETERS,
        returns: "sample count",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48C160 -> sub_46B860: WM_TOUCH history walk; the host delivers no WM_TOUCH, so 0.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_REGISTER_TOUCH_INPUT,
        symbol: "Sys81_18_RegisterTouchInput",
        parameters: SYS81_18_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48C1B0 -> sub_46B660: RegisterTouchWindow when available; 0 on hosts without touch windows.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_COPY_TOUCH_RECORDS,
        symbol: "Sys81_19_CopyTouchRecords",
        parameters: SYS81_19_PARAMETERS,
        returns: "record count",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48C1E0 -> sub_46B6A0: active touch contacts (none without WM_TOUCH).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_SET_CONTROLLER_WAKE_ENTRY,
        symbol: "Sys81_1B_SetControllerWakeEntry",
        parameters: SYS81_1B_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48C210 -> sub_4613A0: index < 0x24 stores dword_565F20[index].",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_QUERY_CONTROLLER_STATE,
        symbol: "Sys81_1D_QueryControllerState",
        parameters: SYS81_1D_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48C250 -> sub_4613C0: DirectInput joystick sum; no DirectInput devices on the host, so 0 and nothing written.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_INJECT_MOUSE_CLICK,
        symbol: "Sys81_1E_InjectMouseClick",
        parameters: SYS81_1E_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48C290 -> sub_46EB30: SendMessageA down/up for buttons 1/2/4/5/6 at the GetCursorPos point (screen coordinates read as client ones).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_OPEN_RESOURCE_STREAM,
        symbol: "Sys81_28_OpenResourceStream",
        parameters: SYS81_28_PARAMETERS,
        returns: "status 0..3",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target sub_4011B0 returns 1 invalid mode, 2 duplicate path, 3 open failure, or 0.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_QUEUE_RESOURCE_CLOSE,
        symbol: "Sys81_29_QueueResourceClose",
        parameters: SYS81_29_PARAMETERS,
        returns: "zero or four",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target queues a close operation; missing handles map to status 4.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_QUEUE_RESOURCE_READ,
        symbol: "Sys81_2A_QueueResourceRead",
        parameters: SYS81_2A_PARAMETERS,
        returns: "zero or four",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target sub_4014A0 queues a read request on the stream worker list.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_QUEUE_RESOURCE_SEEK,
        symbol: "Sys81_2B_QueueResourceSeek",
        parameters: SYS81_2B_PARAMETERS,
        returns: "zero or four",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target sub_4014C0 queues a no-buffer stream operation at the supplied offset.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_GET_FILE_TIMES,
        symbol: "Sys81_2C_GetFileTimes",
        parameters: SYS81_2C_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target opens the file abstraction, queries three FILETIMEs and converts each to SYSTEMTIME.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_SET_FILE_TIMES,
        symbol: "Sys81_2D_SetFileTimes",
        parameters: SYS81_2D_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target converts three SYSTEMTIMEs to FILETIME and applies them to the opened file.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_TEST_PATH_WRITABLE,
        symbol: "Sys81_2F_TestPathWritable",
        parameters: SYS81_2F_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target creates, reopens and deletes a temporary file in the directory.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_READ_RESOURCE_BINARY,
        symbol: "Sys81_30_ReadResourceBinary",
        parameters: SYS81_30_PARAMETERS,
        returns: "sync status or procedure result",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "Target pop order is length, offset, file, archive/root, destination; an empty file name selects the archive's first entry. Named archive reads return success with the actual byte count even when it is smaller than the caller's capacity, while loose-file short reads return status 3. The one-shot async flag chooses synchronous sub_467F50 or DCProcReadBinary.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_INTERNET_READ,
        symbol: "Sys81_31_InternetRead",
        parameters: SYS81_31_PARAMETERS,
        returns: "status -1,0,1,2,3",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target WinINet helper distinguishes initialization, open, read and short-read failures.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_DIRECT_FILE_READ,
        symbol: "Sys81_32_DirectFileRead",
        parameters: SYS81_32_PARAMETERS,
        returns: "status 0,1,8,-1",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target uses unbuffered/direct file I/O and validates a drive-letter path.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_RESOURCE_SIZE,
        symbol: "Sys81_35_ResourceSize",
        parameters: SYS81_35_PARAMETERS,
        returns: "byte count or zero",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target sub_468310 returns the resource size.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_ENUMERATE_DRIVE_TYPES,
        symbol: "Sys81_36_EnumerateDriveTypes",
        parameters: SYS81_36_PARAMETERS,
        returns: "nonzero drive count",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target maps GetDriveTypeA to BGI codes: fixed 1, removable 2, network 3, CD-ROM 4, RAM disk 5.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_GET_DISK_FREE_MB,
        symbol: "Sys81_37_GetDiskFreeMb",
        parameters: SYS81_37_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target queries free bytes and shifts right by 20.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_SHOW_RESOURCE_FILE_DIALOG,
        symbol: "Sys81_38_ShowResourceFileDialog",
        parameters: SYS81_38_PARAMETERS,
        returns: "status",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target builds parallel filter-label and extension arrays then calls the Win32 dialog helper.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_ENUMERATE_RESOURCES,
        symbol: "Sys81_39_EnumerateResources",
        parameters: SYS81_39_PARAMETERS,
        returns: "entry count/status",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target sub_467A60 enumerates matching archive/resource names into packed NUL strings.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_BROWSE_FOLDER,
        symbol: "Sys81_3A_BrowseFolder",
        parameters: SYS81_3A_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target opens SHBrowseForFolder and converts the selected PIDL to a path.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_SHOW_RESOURCE_LIST,
        symbol: "Sys81_3B_ShowResourceList",
        parameters: SYS81_3B_PARAMETERS,
        returns: "selection/status",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target enumerates resources, joins names with newlines and opens a modal list.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_RESOURCE_EXISTS,
        symbol: "Sys81_3C_ResourceExists",
        parameters: SYS81_3C_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target sub_4685D0 tests the resource abstraction rather than only the host filesystem.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_GET_VOLUME_LABEL,
        symbol: "Sys81_3D_GetVolumeLabel",
        parameters: SYS81_3D_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target sub_468670 calls GetVolumeInformationA.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_QUERY_DEVICE_POWER_STATE,
        symbol: "Sys81_3E_QueryDevicePowerState",
        parameters: SYS81_3E_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target returns whether the query mechanism opened/succeeded and writes the device power state.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_CREATE_CHILD_THREAD,
        symbol: "Sys81_44_CreateChildThread",
        parameters: SYS81_44_PARAMETERS,
        returns: "integer thread ID",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target constructs DCTChildThread, validates sizes/index and returns CThread+8.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_CONFIGURE_SCREEN_MODE,
        symbol: "Sys81_60_ConfigureScreenMode",
        parameters: SYS81_60_PARAMETERS,
        returns: "status 0,1,2",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48CBC0 -> sub_4615A0: index >= 8 -> 1, zero dimension -> 2, else the size table entry.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_QUERY_EFFECTIVE_DISPLAY_MODE,
        symbol: "Sys81_61_QueryEffectiveDisplayMode",
        parameters: NO_PARAMETERS,
        returns: "mode 0..2",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48CC00 -> sub_45EDE0: full-screen scaling mode; 2 falls back to 0 when the adjusted desktop is smaller than the mode.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_SET_MONITOR_ADAPTER_MODE,
        symbol: "Sys81_62_SetMonitorAdapterMode",
        parameters: SYS81_62_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48CC20 -> sub_45ED90: dword_50721C (initially 1): full screen on the window's monitor (1) or adapter 0.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_SET_CONFIG_INPUT_MODE,
        symbol: "Sys81_63_SetConfigInputMode",
        parameters: SYS81_63_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48CC50 -> sub_45EDC0: dword_565EAC full-screen scaling 0 fit, 1 stretch, 2 unscaled (renderer viewport, sub_45F110).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_CONFIGURE_LOGICAL_SCREEN_SIZE,
        symbol: "Sys81_64_ConfigureLogicalScreenSize",
        parameters: SYS81_64_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48CC80 -> sub_45ECD0: windowed client size, zero dimension follows the mode; resizes a window at once.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_SET_WINDOW_POSITION_OVERRIDE,
        symbol: "Sys81_65_SetWindowCenterLock",
        parameters: SYS81_65_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_48CCB0 -> sub_49ADC0 stores dword_566A74 (.bss, initially 0). While non-zero, WM_WINDOWPOSCHANGING of a windowed (not full-screen) game window replaces the position with the centred one (sub_461B40); hosts re-centre on every move.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_SWAP_PAUSE_ON_DEACTIVATE,
        symbol: "Sys81_68_SwapPauseOnDeactivate",
        parameters: SYS81_68_PARAMETERS,
        returns: "previous value",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_48CCD0 -> sub_499370: swaps dword_566A60 and pushes the old value. While set, WM_ACTIVATE(inactive) freezes the engine clock (sub_4992E0) and pauses DirectShow movies; activation resumes.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_SET_PRINT_SCREEN_HOTKEY_CAPTURE,
        symbol: "Sys81_69_SetPrintScreenHotkeyCapture",
        parameters: SYS81_69_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48CD00 -> sub_49AE40: RegisterHotKey for PrintScreen; nothing script-visible on the host.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_SET_ERROR_CAPTURE_MODE,
        symbol: "Sys81_6A_SetErrorCaptureMode",
        parameters: SYS81_6A_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target sub_464440 selects whether engine error text is captured.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_COPY_CAPTURED_ERROR,
        symbol: "Sys81_6B_CopyCapturedError",
        parameters: SYS81_6B_PARAMETERS,
        returns: "byte length including NUL",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target sub_464480 copies dword_566004 when non-null and returns strlen+1.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_QUERY_PIXEL_SHADER_VERSION,
        symbol: "Sys81_6D_QueryPixelShaderVersion",
        parameters: NO_PARAMETERS,
        returns: "low 16-bit D3D shader version",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48CD70: LOWORD(D3DCAPS9 PixelShaderVersion), 0x0300 for the renderer.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_UNCAUGHT_EXCEPTION_ACTIVE,
        symbol: "Sys81_6E_UncaughtExceptionActive",
        parameters: NO_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48CD90 -> sub_460B00: whether d3dx9_43 D3DXCreateEffect loaded; the host has no D3DX.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_SET_BICUBIC_SHADER,
        symbol: "Sys81_6F_SetBicubicShader",
        parameters: SYS81_6F_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48CDB0 -> sub_460AB0: 0 drops the bicubic effect (1); 1 needs ps_3_0 and D3DX (sub_460980), so 0 on the host.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_WIDE_STRING_DISTANCE,
        symbol: "Sys81_B0_WideStringDistance",
        parameters: SYS81_B0_PARAMETERS,
        returns: "insertion/deletion distance or -1",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48CDE0 -> sub_496770: LCS length of two UTF-16 strings (unit-cost grid walk); -1 for null, 0 for an empty second string.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_SHIFT_JIS_TO_UTF16,
        symbol: "Sys81_B7_ShiftJisToUtf16",
        parameters: SYS81_B7_PARAMETERS,
        returns: "UTF-16 unit count or -1",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48CE10 -> sub_496700: Shift-JIS characters as 16-bit units (double-byte lead << 8 | trail); returns the count, -1 for null.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_BLOB_TABLE_OPEN,
        symbol: "Sys81_D0_BlobTableOpen",
        parameters: SYS81_D0_PARAMETERS,
        returns: "zero or 0x80000001",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48CE40 -> sub_46AE10: capacity <= 1 -> 0x80000001, else new table, handle ++dword_566684.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_BLOB_TABLE_CLOSE,
        symbol: "Sys81_D1_BlobTableClose",
        parameters: SYS81_D1_PARAMETERS,
        returns: "zero or 0x80000002",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48CE80 -> sub_46AEC0: 0 or 0x80000002.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_BLOB_TABLE_STORE,
        symbol: "Sys81_D2_BlobTableStore",
        parameters: SYS81_D2_PARAMETERS,
        returns: "status",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48CEB0 -> sub_46AF30: store at index (or first free with an index output); capacity doubles to fit; size 0 -> 0x80000003.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_BLOB_TABLE_REMOVE,
        symbol: "Sys81_D3_BlobTableRemove",
        parameters: SYS81_D3_PARAMETERS,
        returns: "status",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48CF10 -> sub_46AF90: free a slot; 0x80000004 when empty.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_BLOB_TABLE_FETCH,
        symbol: "Sys81_D4_BlobTableFetch",
        parameters: SYS81_D4_PARAMETERS,
        returns: "status",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48CF50 -> sub_46AFC0: copy (non-null buffer) and size; 0x80000004 when empty.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_BLOB_TABLE_ENUMERATE,
        symbol: "Sys81_D5_BlobTableEnumerate",
        parameters: SYS81_D5_PARAMETERS,
        returns: "status",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48CFA0 -> sub_46B000: (index, size) of stored slots and their count.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_LAUNCH_PROCESS_WAIT,
        symbol: "Sys81_E0_LaunchProcessWait",
        parameters: SYS81_E0_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48CFE0 -> sub_472A10(wait, retry): dir\\\\command or root + command, waits and writes the exit code; failure retries through OK|CANCEL and the quit question.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_UPDATE_ROLLING_HASH,
        symbol: "Sys81_E9_UpdateRollingHash",
        parameters: SYS81_E9_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48D060 -> sub_401900: rolling hash update (state*233 + byte, byte lanes).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_MD5,
        symbol: "Sys81_EA_Md5",
        parameters: SYS81_EA_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48D0A0 -> sub_4A1FB0: MD5 of the buffer.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_CREATE_NAMED_MUTEX,
        symbol: "Sys81_EC_CreateNamedMutex",
        parameters: SYS81_EC_PARAMETERS,
        returns: "integer handle or zero",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48D0E0 -> sub_490470: CreateMutexA(TRUE); 0 when it exists in any process (host file lock), else ++dword_56697C.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_RELEASE_NAMED_MUTEX,
        symbol: "Sys81_ED_ReleaseNamedMutex",
        parameters: SYS81_ED_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48D110 -> sub_490500: release and close; 1 for a known handle.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_RUN_INSTALLATION_PROCEDURE,
        symbol: "Sys81_F2_RunInstallationProcedure",
        parameters: SYS81_F2_PARAMETERS,
        returns: "no immediate value on success; immediate -1/1/2/3 on initialization failure",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "Target sub_48C650 converts two zero-terminated directory lists, one DWORD descriptor array and two parallel file-name arrays, then constructs DCProcInstallation. Status zero installs the procedure; 0x80000000..2 map to immediate 1..3. The portable host performs the same file/directory setup synchronously and releases the VM through a target-shaped DCProcInstallation boundary.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS81_VALIDATE_OR_CREATE_USER_PATH,
        symbol: "Sys81_F7_CreateSpecialFolderShortcut",
        parameters: SYS81_F7_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Wrapper 0x0048C8D0 forwards a special-folder selector in ECX plus optional group, shortcut name, and target to sub_4713F0; the portable host creates a native or desktop-compatible shortcut in the selected folder.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SRAND,
        symbol: "Sys80_00_Srand",
        parameters: RNG_SEED_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_488820: srand(value) on the main thread's MSVC CRT state, shared with every native rand() caller (particles, Sys80:01/02).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_RAND,
        symbol: "Sys80_01_Rand",
        parameters: NO_PARAMETERS,
        returns: "Microsoft CRT rand value",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_488840: pushes MSVC rand() (holdrand*214013+2531011, bits 16..30; initial seed 1).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_RAND_MAX,
        symbol: "Sys80_02_RandMax",
        parameters: RAND_MAX_PARAMETERS,
        returns: "value in 0..max_exclusive-1, or zero for max<=0",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_488860: for n > 0 returns (((rand()<<8)^rand())<<8^rand()) % n with the three rand() calls in that order; n <= 0 returns 0 without calling rand().",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_GET_ENGINE_TICK,
        symbol: "Sys80_04_GetEngineTick",
        parameters: NO_PARAMETERS,
        returns: "engine milliseconds",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_4888B0 -> sub_499290: wrapping 32-bit millisecond clock; host jumps backwards or over 500 ms (dword_503DE8) are excluded, and the value is frozen inside a pause section that froze it (deactivation with Sys81:68, modal dialogs). Interactive runs start at the host uptime, deterministic replays at 0.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_QUERY_PERFORMANCE_COUNTER_NS,
        symbol: "Sys80_05_QueryPerformanceCounterNs",
        parameters: PERFORMANCE_COUNTER_PARAMETERS,
        returns: "boolean success",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_4888D0 -> sub_499430: QueryPerformanceCounter/frequency*1e9 as a u64 written low then high DWORD; returns 1. Not stopped by clock pauses; interactive runs read the host monotonic clock, replays derive it from the engine clock.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SET_PERFORMANCE_PROFILING,
        symbol: "Sys80_06_SetPerformanceProfiling",
        parameters: PERFORMANCE_PROFILING_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_488900 -> sub_401600: stores the raw flag; non-zero resets count/total/within-budget/pending and sets the budget to 96% of one display refresh (sub_46F8E0, refresh rate dword_5177C8 or 60). Samples (sub_4017F0/sub_401830) time the update pass (sub_48D480, kept pending) plus scene composition before Present (sub_462010/sub_462050); Present and its vsync wait are not measured. Nanoseconds stand in for the QueryPerformanceCounter frequency; without a raster status (sub_45FBF0 failure) only the time test decides within-budget.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_READ_PERFORMANCE_METRIC,
        symbol: "Sys80_07_ReadPerformanceMetric",
        parameters: PERFORMANCE_METRIC_PARAMETERS,
        returns: "void; metric is written through output_ptr",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_488920 -> sub_401670 pops the metric then the output pointer: 0 sample count; 1 total/(freq*count)*1e6; 2 (budget>>1)*count/total*10000 when total > 0; 3 within^2/count^2*10000; other selectors and empty states write 0. x87 results are truncated through __int64 to a DWORD.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_READ_CURSOR_POINT,
        symbol: "Sys80_08_ReadCursorPoint",
        parameters: NO_PARAMETERS,
        returns: "two values: client-space x, then y",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_488950 -> sub_48F180: the active touch contact (dword_5666B0), else the cursor in client space mapped to game space by sub_45EE30 with (int) truncation; (0, 0) before the window exists; pushes x then y. Hosts deliver cursor positions already in game space; the target window cannot be resized (sub_461C40), the portable window can, with the same mapping.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_QUERY_PRESENTATION_STATE,
        symbol: "Sys80_09_QueryPresentationState",
        parameters: NO_PARAMETERS,
        returns: "renderer presentation state/timestamp",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_488990 -> sub_460450 returns dword_565ED8, which sub_460460 sets to the engine clock (sub_499290) after a successful Present; 0 before the first frame.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_COPY_GRAPHICS_CAPABILITIES,
        symbol: "Sys80_0A_CopyGraphicsCapabilities",
        parameters: GRAPHICS_CAPABILITY_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_4889B0 copies 0x40 bytes from dword_518930 (.bss), which sub_46F100 fills at startup: [0] vendor index, [1] family (extended bits OR-ed in for 0/15), [2] model with extended model, [3] stepping, [4] brand byte (Intel) or CPUID 0x80000001 EBX word, [5]/[6]/[7] L1/L2/L3 words from the CPUID 2 descriptor table and CPUID 4 levels (Intel) or CPUID 0x80000005/6, [8] RDTSC MHz over one second, [9] processor count; a failed CPUID query returns early. Hosts without CPUID keep the zero record (cpu_record.rs).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_QUERY_GRAPHICS_MEMORY_METRIC,
        symbol: "Sys80_0B_QueryGraphicsMemoryMetric",
        parameters: NO_PARAMETERS,
        returns: "renderer-owned capacity/memory field",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_4889E0 -> sub_443540 returns CObjectManager+0x58, set by sub_46FB30 at startup (display mode 2) and on every display-mode change (sub_461670) from the processor record and that mode's size (dword_506B8C/dword_506BAC), in the target's 32-bit arithmetic.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_GET_LOCAL_TIME,
        symbol: "Sys80_0C_GetLocalTime",
        parameters: SYSTEM_TIME_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_488A00: GetLocalTime into the 16-byte SYSTEMTIME (year, month, weekday, day, hour, minute, second, ms), taken from one host time sample.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_GET_PHYSICAL_MEMORY,
        symbol: "Sys80_0D_GetPhysicalMemory",
        parameters: NO_PARAMETERS,
        returns: "two values: total physical bytes, then available physical bytes",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_488A20: GlobalMemoryStatus total then available physical bytes, each capped at 0x7FFFFFFF; pushes total, then available.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_QUERY_WINDOW_MINIMIZE_LATCH,
        symbol: "Sys80_0E_QueryWindowMinimizeLatch",
        parameters: NO_PARAMETERS,
        returns: "boolean minimize/restore latch",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_488AB0 -> sub_49ADB0 returns dword_566708: Sys80:65 sets it; WM_ACTIVATE clears it once the window is active and no longer iconic.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_QUERY_WINDOW_ACTIVE,
        symbol: "Sys80_0F_QueryWindowActive",
        parameters: NO_PARAMETERS,
        returns: "boolean active-window state",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_488AD0 -> sub_49ADA0: dword_5666FC, the last WM_ACTIVATE state.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_RESET_INPUT_CONFIGURATION,
        symbol: "Sys80_10_ResetInputConfiguration",
        parameters: INPUT_CONFIGURATION_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_488AF0: stores the input-configuration flag (dword_506A44) and clears fields 0/1/2/4 of every input record (sub_46DF80); the field-3 accumulator read by Sys80:12 and field 5 survive.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_QUERY_KEY_DOWN,
        symbol: "Sys80_11_QueryKeyDown",
        parameters: KEY_STATE_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_488B10 -> sub_46DAC0: bit 15 of GetAsyncKeyState for the logical key; with mouse mapping mode 1 (dword_566930) left reads left|right and right reads nothing; the system button swap is compensated; reports 0 unless the window is active or background polling (dword_566800) is on.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SUM_INPUT_DESCRIPTOR_STATE,
        symbol: "Sys80_12_SumInputDescriptorState",
        parameters: INPUT_DESCRIPTOR_LIST_PARAMETERS,
        returns: "sum of current descriptor count/state fields",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_488B40: sums the field-3 accumulator (dword_518CB4[6*d], sub_46E1D0) over a zero-terminated contiguous DWORD array of descriptors; nothing is consumed.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SET_MOUSE_BUTTON_MAPPING_MODE,
        symbol: "Sys80_1E_SetMouseButtonMappingMode",
        parameters: MOUSE_BUTTON_MAPPING_PARAMETERS,
        returns: "boolean accepted",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_488DF0 -> sub_48F950: mode <= 1 (unsigned) is stored as the mouse mapping mode (dword_566930: 1 turns the right button into the left) and 1 is pushed; other values push 0.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_ALLOC,
        symbol: "Sys80_20_Alloc",
        parameters: ALLOC_PARAMETERS,
        returns: "opaque BP pointer",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target 0x00488380 validates the requested size, allocates through sub_48DC70's memory-class table and raises the native VM error path when allocation returns zero.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_FREE,
        symbol: "Sys80_21_Free",
        parameters: FREE_PARAMETERS,
        returns: "boolean success",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target 0x004883D0 treats null as success; non-null handles are decoded by sub_48DD10 and invalid/double-free values enter the native VM error path.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_COUNT_FILES,
        symbol: "Sys80_24_CountFiles",
        parameters: COUNT_FILES_PARAMETERS,
        returns: "matched file count",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_488F50 -> sub_467110(no buffer, no limit): counts files matching the FindFirstFileA pattern (DOS wildcard rewriting, case-insensitive), then with the recursive flag every subdirectory's matches; recursion builds patterns from strrchr(pattern, '\\\\'), so a pattern without a backslash finds nothing below.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_ENUMERATE_FILES,
        symbol: "Sys80_25_EnumerateFiles",
        parameters: ENUMERATE_FILES_PARAMETERS,
        returns: "entry count, required byte count when destination is null, or -1 on overflow",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_488F90 (pops max count, recursive, pattern, capacity, buffer) -> sub_467110: matching files in NTFS order (upcased UTF-16), up to the count, packed as C strings, then each subdirectory recursively with prefix 'sub\\\\' and the remaining count (0 means unlimited, as the target's unsigned difference); a name that does not fit returns -1 after the earlier ones were copied; a null buffer returns the byte total, otherwise the count. No recursion without a backslash in the pattern.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_ENUMERATE_DIRECTORIES,
        symbol: "Sys80_26_EnumerateDirectories",
        parameters: ENUMERATE_DIRECTORIES_PARAMETERS,
        returns: "directory count, required byte count when destination is null, or -1 on overflow",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_489010 -> sub_467460: immediate subdirectories other than . and .. in NTFS order, up to the count, packed as C strings; overflow returns -1; a null buffer returns the byte total, otherwise the count.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_MOVE_FILE,
        symbol: "Sys80_27_MoveFile",
        parameters: TWO_PATH_PARAMETERS,
        returns: "Win32 BOOL",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_489060: MoveFileA(last argument, first argument) without flags: fails when the destination exists (except a case-only rename), does not create its directory, moves files across volumes by copy and delete, directories only within a volume.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_CREATE_DIRECTORY,
        symbol: "Sys80_28_CreateDirectory",
        parameters: PATH_PARAMETER,
        returns: "Win32 BOOL",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_489090: CreateDirectoryA creates one level; an existing path or a missing parent fails.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_REMOVE_DIRECTORY,
        symbol: "Sys80_29_RemoveDirectory",
        parameters: PATH_PARAMETER,
        returns: "Win32 BOOL",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_4890C0: RemoveDirectoryA removes an empty directory only.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_DIRECTORY_EXISTS,
        symbol: "Sys80_2A_DirectoryExists",
        parameters: PATH_PARAMETER,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_4890F0: bit 4 (DIRECTORY) of GetFileAttributesA, 0 when the path does not exist.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SPLIT_PATH,
        symbol: "Sys80_2B_SplitPath",
        parameters: SPLIT_PATH_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_489130 pops the path, then the ext, name, dir and drive outputs; a non-null path is split as wide text (sub_42D7F0: MultiByteToWideChar(CP_ACP), _wsplitpath: drive 'X:', directory through the last '\\\\' or '/', name, extension from the last '.') and each non-null output receives its part; returns 1, or 0 without writes for a null path.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_GET_FILE_ATTRIBUTES,
        symbol: "Sys80_2C_GetFileAttributes",
        parameters: PATH_PARAMETER,
        returns: "raw u32 Win32 attributes or 0xFFFFFFFF",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_4891A0: GetFileAttributesA unchanged (-1 when missing): READONLY from the host permission, DIRECTORY from the type, the other settable bits as stored by Sys80:2D (default ARCHIVE for files), NORMAL when nothing remains.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SET_FILE_ATTRIBUTES,
        symbol: "Sys80_2D_SetFileAttributes",
        parameters: SET_ATTRIBUTES_PARAMETERS,
        returns: "Win32 BOOL",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_4891D0 pops the attribute mask, then the path: SetFileAttributesA keeps READONLY/HIDDEN/SYSTEM/ARCHIVE/TEMPORARY/OFFLINE/NOT_CONTENT_INDEXED (NORMAL alone clears them); READONLY maps to the host permission, the rest persists in an extended attribute.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_COPY_FILE,
        symbol: "Sys80_2F_CopyFile",
        parameters: TWO_PATH_PARAMETERS,
        returns: "Win32 BOOL",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_489200: clears READONLY on an existing destination (second-to-last argument), then CopyFileA(last argument, destination, FALSE): overwrite, data plus attributes (with ARCHIVE) and last-write time; a directory source fails.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_READ_FILE_BYTES,
        symbol: "Sys80_30_ReadFileBytes",
        parameters: READ_FILE_PARAMETERS,
        returns: "byte count, or zero after target retry/error handling",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_489250 -> sub_466010 (pops file, archive, destination): clears 16 bytes, reads the loose file below the primary root (then the Sys80:36/37 search directories), decoded by sub_465880 (DSC, CBG v1/v2, raw), else with a non-null archive the entry of primary_root+archive (composite archives search their members); a miss retries the media root and then goes to sub_465F80, which is a script error ('指定されたファイル [ a : f ] は存在しません') unless a media root exists (then the OK/Cancel media prompt loops); returns the decoded size, 0 for other archive errors. Without an archive the media root is tried for the loose file. See src/engine/resource_io.c.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_READ_FILE_RANGE,
        symbol: "Sys80_31_ReadFileRange",
        parameters: READ_FILE_RANGE_PARAMETERS,
        returns: "status 0,1,2,3,5,or 6",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_489290 -> sub_466190 (pops length, offset, file, archive, destination): loose read under the primary root and search directories, then the media root (null archive) or primary_root+archive and media_root+archive; range per sub_465880 (both 0 = whole, count 0 or beyond the size 3, end beyond the size 2); returns 0, 1 missing, 2, 3, 5 read/decode failure, 6 over 64 MiB; never fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_WRITE_FILE_BYTES,
        symbol: "Sys80_32_WriteFileBytes",
        parameters: WRITE_FILE_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target 0x004887F0 returns whether sub_465E30 wrote exactly the requested byte count.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_DELETE_FILE,
        symbol: "Sys80_33_DeleteFile",
        parameters: DELETE_FILE_PARAMETERS,
        returns: "Win32 BOOL",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_488840 pops file, then root; it deletes root\\\\file, or primary root + file when the root is null (DeleteFileA), and pushes the success flag.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_FILE_EXISTS,
        symbol: "Sys80_34_FileExists",
        parameters: FILE_EXISTS_PARAMETERS,
        returns: "target search-root status",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_4893C0 -> sub_466B20 (pops file, then archive): with an archive, a loose non-directory below the primary root or search directories, else (names of 0x60+ bytes are fatal) the entry in primary_root+archive or media_root+archive; without, an absolute name is tested directly, otherwise the loose file below the primary or media root.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_FILE_SIZE,
        symbol: "Sys80_35_FileSize",
        parameters: FILE_SIZE_PARAMETERS,
        returns: "byte count or zero",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_489400 -> sub_466840 (pops file, then archive): decoded size of the loose file, else of the archive entry (media root only after ARCHIVE_NOT_FOUND), else 0; without an archive the media root's loose file.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SET_ADDITIONAL_RESOURCE_SEARCH,
        symbol: "Sys80_36_SetAdditionalResourceSearch",
        parameters: RESOURCE_SEARCH_ENABLE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target sub_465240 stores the value in dword_506BE0; it is a mode flag, not a commit operation.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_PREPEND_RESOURCE_SEARCH_PATH,
        symbol: "Sys80_37_PrependResourceSearchPath",
        parameters: RESOURCE_SEARCH_PATH_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target allocates a path node and prepends it to the linked list rooted at dword_566630.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_REGISTER_COMPOSITE_ARCHIVE,
        symbol: "Sys80_38_RegisterCompositeArchive",
        parameters: COMPOSITE_ARCHIVE_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target consumes a zero-terminated pointer array and constructs/registers a named DCArchiveComplex.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SET_VALIDATED_FILE_ROOT,
        symbol: "Sys80_39_SetValidatedFileRoot",
        parameters: PATH_PARAMETER,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target accepts an existing directory and stores it with a trailing backslash in byte_518978.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_GET_SPECIAL_FOLDER,
        symbol: "Sys80_3A_GetSpecialFolder",
        parameters: SPECIAL_FOLDER_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target selects six Win32 system/shell folder sources, including ProgramFilesDir from the registry, and copies the resulting path.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_OPEN_FILE_DIALOG,
        symbol: "Sys80_3B_OpenFileDialog",
        parameters: FILE_DIALOG_PARAMETERS,
        returns: "0 success, -1 cancel, or validation status",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target builds one extension filter and calls GetOpenFileNameA/GetSaveFileNameA. The portable host now invokes a blocking native/scripted chooser on macOS, Windows and Linux while preserving target status codes.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_REQUIRE_RESOURCE_FILE,
        symbol: "Sys80_3C_RequireResourceFile",
        parameters: REQUIRE_RESOURCE_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target repeatedly searches configured roots and prompts Retry/Quit. The portable host performs the same loop through native macOS, Windows, or Linux message dialogs.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_GET_CONFIGURED_ROOT,
        symbol: "Sys80_3D_GetConfiguredRoot",
        parameters: CONFIGURED_ROOT_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target 0x00488B20/sub_4649A0 copies the primary root for kind 0, or the configured secondary root for kind 1; other values fail.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SET_PRIMARY_ROOT,
        symbol: "Sys80_3E_SetPrimaryRoot",
        parameters: PATH_PARAMETER,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target 0x00488B50 accepts only an existing directory, writes path plus a trailing backslash to the primary global root buffer and returns one.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_CONFIGURE_REMOVABLE_ARCHIVE,
        symbol: "Sys80_3F_ConfigureRemovableArchive",
        parameters: REMOVABLE_ARCHIVE_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target scans removable/CD drives for the configured archive marker and stores the successful drive/subdirectory as the secondary root. The portable host scans platform mount roots and preserves the retry loop.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_LOAD_PROGRAM_MODULE,
        symbol: "Sys80_40_LoadProgramModule",
        parameters: LOAD_PROGRAM_MODULE_PARAMETERS,
        returns: "integer code-region base offset",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_488C00 pops file then archive, decodes the module (sub_465AB0; a read/decode failure is fatal) and sub_444CE0 appends its code to the current CThread when it fits the code region (fatal otherwise). The value pushed is the code offset where the module starts (the previous code-used end). The target's disk-swap retry dialog for a missing archive is not reproduced; the portable runtime fails at once.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_FREE_LAST_PROGRAM_MODULE,
        symbol: "Sys80_41_FreeLastProgramModule",
        parameters: FREE_PROGRAM_ABI_SLOT_PARAMETERS,
        returns: "remaining module count",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_488CD0 pops nothing. sub_444D80 unlinks the newest module record, rolls back the code-used end and pushes the remaining module count (the main program counts); with no record it pushes 0x80000001, and a remaining count of 0 is a fatal error.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_LOAD_PROGRAM_THREAD,
        symbol: "Sys80_44_LoadProgramThread",
        parameters: LOAD_PROGRAM_THREAD_PARAMETERS,
        returns: "integer CThread identifier",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_488D00 pops region p1, region p2, operand slots, file, archive. sub_48D080 reads the file (sub_465AB0; unreadable is fatal), appends a CThread at the end of the thread chain (sub_444A70 -> sub_4447C0: 4*slots operand bytes, p2 code region, p1 second region), loads the module into the code region (sub_444CE0; a module that does not fit is fatal) and returns the new thread id (CThread+8).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_RTC_NOOP,
        symbol: "Sys80_45_RtcNoop",
        parameters: NO_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "The release-image selector table points at __RTC_NumErrors, but the BP ABI exposes no output and no useful engine side effect.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_CURRENT_THREAD_ID,
        symbol: "Sys80_46_CurrentThreadId",
        parameters: NO_PARAMETERS,
        returns: "integer CThread identifier",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target sub_488D80 returns the current CThread field at +8.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_THREAD_EXISTS,
        symbol: "Sys80_47_ThreadExists",
        parameters: THREAD_ID_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target recursively searches the CThread tree; identifier zero is never reported as present.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_INPUT_MESSAGE_SERIAL,
        symbol: "Sys80_13_GetInputMessageSerial",
        parameters: NO_PARAMETERS,
        returns: "monotonic input/window-message serial",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_488B80 -> sub_46EB20: dword_566850, incremented (sub_46EB10) by WM_CLOSE, every left/right/middle/X button down (before Knob capture), mouse wheel and horizontal wheel, and the press edges of WM_KEYDOWN, F10 and Alt.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SET_INPUT_MASTER_GATE,
        symbol: "Sys80_14_SetInputMasterGate",
        parameters: INPUT_GATE_PARAMETER,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_488BA0 -> sub_46DEE0: stores the skip-gate master flag dword_506A48 read by Sys80:17.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SET_INPUT_LATCHED_STATE,
        symbol: "Sys80_15_SetInputLatchedState",
        parameters: INPUT_GATE_PARAMETER,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_488BC0 -> sub_46DEF0: stores the latched skip state dword_56683C read by Sys80:17.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SAMPLE_CONFIGURED_INPUT,
        symbol: "Sys80_16_SampleConfiguredInput",
        parameters: NO_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_488BE0 -> sub_46DF00: sets the poll flag dword_566840 and drains (sub_46E0A0) every descriptor of the 0x80000000 list dword_5069F8 (default {17}).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_QUERY_CONFIGURED_INPUT_GATE,
        symbol: "Sys80_17_QueryConfiguredInputGate",
        parameters: NO_PARAMETERS,
        returns: "configured input event/status",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_488BF0 -> sub_46E390: starting from the latched state, while the window is active and input configuration is on, walks dword_5069F8: a physically held key (sub_46DAC0) marks any-down; with the poll flag a draining hit latches, otherwise a held key returns at once; hits count only while the window is not minimized (dword_566700). The poll flag drops once nothing is held. Returns 1 when latched and the master gate (dword_506A48) is set.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_REGISTER_INPUT_SCOPE,
        symbol: "Sys80_18_RegisterInputScope",
        parameters: INPUT_SCOPE_PARAMETER,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_488C10: packed = (scope<<16)|0xFFFF is inserted into the pointer chain as a whole-plane node (sub_46DC20, rect dword_506A4C) and into the keyboard chain (sub_46DC80), each before the first node whose scope is not greater, then one stateful sub_46E460 query whose result is discarded.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_QUERY_AND_UNREGISTER_INPUT_SCOPE,
        symbol: "Sys80_19_QueryAndUnregisterInputScope",
        parameters: INPUT_SCOPE_PARAMETER,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_488C50: one stateful sub_46E460 query, then removes the first pointer-chain node (sub_46DD00) and keyboard-chain node (sub_46DD10) with that packed scope.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_QUERY_INPUT_EVENT_BITS,
        symbol: "Sys80_1A_QueryInputEventBits",
        parameters: QUERY_INPUT_EVENT_BITS_PARAMETERS,
        returns: "current input-event bitmap",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_488C90 -> sub_46E460((scope<<16)|0xFFFF): when the keyboard chain accepts the scope (sub_46DD70: input on and scope >= the top keyboard node), the nineteen class tables (sub_46E1E0; each drains descriptors until one hits) and then the skip gate (sub_46E390, bit 31); when the pointer chain accepts it (sub_46DD90), descriptors 1/2/4/5/6 drain into bits 1/2/4/0x10/0x20.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_REGISTER_INPUT_CLASS_DESCRIPTORS,
        symbol: "Sys80_1B_RegisterInputClassDescriptors",
        parameters: REGISTER_INPUT_CLASS_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_488CC0 -> sub_46E500: pops the descriptor pointer, then the class; a list of 16 or more entries, then a class other than 0x40/0x80/0x100/0x200/0x1000/0x2000/0x4000/0x8000/0x40000000/0x80000000, is a script error; the zero-terminated list is copied into that fixed table (classes 0x100/0x200 are also the child lists of aggregate descriptors 193/194).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_QUERY_INPUT_CLASS_LEVEL,
        symbol: "Sys80_1C_QueryInputClassLevel",
        parameters: QUERY_INPUT_CLASS_LEVEL_PARAMETERS,
        returns: "current input-class level/count",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_488D60 -> sub_46E5D0: wrapping sum of the field-3 accumulators of every descriptor in the selected class tables (1/2/4/0x10/0x20 and the nineteen classes; the 0x80000000 list is excluded).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_QUERY_SCOPED_INPUT_EVENT,
        symbol: "Sys80_1D_QueryScopedInputEvent",
        parameters: QUERY_SCOPED_INPUT_EVENT_PARAMETERS,
        returns: "drained event count with 0x80000000 first-down flag",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_488D90 pops the descriptor, then the scope; descriptors 1/2 test the pointer chain (sub_46DD90), others the keyboard chain (sub_46DD70), and an accepted scope drains the descriptor (sub_46E0A0: lazy release, count, first-held bit 31).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_CONFIGURE_CURSOR_MOTION,
        symbol: "Sys80_1F_ConfigureCursorMotion",
        parameters: CONFIGURE_CURSOR_MOTION_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_488E20 -> sub_48F280 (script order x, y, interpolation, duration, ups, cancel): ignored while iconic; steps = ups*duration/1000 (u32), at least 1; sub_48F430 then, each frame while the window is active, emits every step whose deadline start + duration*(step+1)/steps (u32) has passed, at start + sub_48F3D0(delta) (linear 16.16 or integer-angle cosine), the last step exactly at the target; with the cancel flag a cursor that moved more than ceil(presentation scale) from the last generated point stops it; an inactive window drops it.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SET_MESSAGE_AUXILIARY_INPUT_MASK,
        symbol: "Sys81_1F_SetMessageAuxiliaryInputMask",
        parameters: MESSAGE_AUXILIARY_INPUT_MASK_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48C2C0 -> sub_431DC0: dword_507690 (initially 0x2000), the extra input bits of CProcDspMsg/CProcedure.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_ENQUEUE_MESSAGE,
        symbol: "Sys80_48_EnqueueMessage",
        parameters: ENQUEUE_MESSAGE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target slot 0x00504420 calls BPThread_EnqueueDword and appends one value to CThread+0x5C/+0x60 FIFO state.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_DEQUEUE_MESSAGE,
        symbol: "Sys80_49_DequeueMessage",
        parameters: DEQUEUE_MESSAGE_PARAMETERS,
        returns: "boolean success",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target slot 0x00504424 calls BPThread_DequeueDword, writes one value through output_ptr, and reports whether a value was available.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_ENQUEUE_MESSAGE_ARRAY,
        symbol: "Sys80_4A_EnqueueMessageArray",
        parameters: ENQUEUE_MESSAGE_ARRAY_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target handler appends an array by repeatedly invoking the confirmed one-value FIFO enqueue helper.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_DEQUEUE_MESSAGE_ARRAY,
        symbol: "Sys80_4B_DequeueMessageArray",
        parameters: DEQUEUE_MESSAGE_ARRAY_PARAMETERS,
        returns: "actual dequeued count",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target handler performs bounded FIFO dequeue and returns the number written to output_ptr.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_INVOKE_THREAD_CALLBACK,
        symbol: "Sys80_4C_InvokeThreadCallback",
        parameters: THREAD_CALLBACK_PARAMETERS,
        returns: "callback presence/invocation status",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_488FC0: finds the thread with sub_444B90 (id 0, the empty root CThread, is never found; the script thread is 1) and queues {a, b, c} on its current procedure (sub_445230 -> sub_431B40); pushes 1 when queued, 0 for an unknown thread or no procedure.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SET_SYSTEM_WAIT_STATE,
        symbol: "Sys80_50_SetSystemWaitState",
        parameters: SYSTEM_WAIT_STATE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target 0x00489030 pops one i32 and sub_4319B0 stores it unchanged in dword_507688. The native C return value 1 is not a BP stack output.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_WAIT_WINDOW_MESSAGE,
        symbol: "WaitWndMsg",
        parameters: UNRECOVERED_SINGLE_PARAMETER,
        returns: "on completion pushes lParam and wParam",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "Target sub_489080 pops the message id, allocates 0x24 bytes, constructs CProcWaitWndMsg through sub_43D4D0, and registers the waiter through sub_499F40. The virtual poll consumes the matching signaled waiter and pushes lParam followed by wParam.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SET_THREAD_TIMER,
        symbol: "SetThreadTimer",
        parameters: UNRECOVERED_SINGLE_PARAMETER,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_489C00 -> sub_445660: CThread+0x84 = sub_499290() + duration with 32-bit wrap (negative durations are not clamped).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SET_AND_QUERY_THREAD_TIMER,
        symbol: "Sys80_59_SetAndQueryThreadTimer",
        parameters: THREAD_TIMER_DURATION_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_489C20: CThread+0x84 += duration (sub_445680), then pushes whether sub_445630 (signed deadline - now, 0 when not positive) is non-zero.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_WAIT_THREAD_TIMER,
        symbol: "WaitTiming",
        parameters: NO_PARAMETERS,
        returns: "boolean: remaining thread timer was positive",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "BGI 1.622 sub_489C60: remaining = sub_445630 (signed CThread+0x84 - now, 0 when not positive); a positive value installs CProcWaitTiming with deadline now + remaining (unsigned completion test deadline <= now, sub_431E30); pushes remaining != 0 and returns status 2.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_WAIT_TIMING_EX,
        symbol: "WaitTimingOrInput",
        parameters: WAIT_TIMING_PARAMETERS,
        returns: "procedure completion result",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "Target sub_4891F0 pops scope, input flag, duration and installs CProcWaitTimingEx (sub_43D2E0): deadline = engine tick + duration in the procedure (not the 0x80:0x5A thread timer); with input enabled the packed scope is registered like 0x80:0x18 and primed. Tick sub_43D430 pushes 0 on deadline, cleared dword_507688, callback code 0 or 1, and 1 on input bits & (dword_507690|0x80000181); the destructor unregisters the scope.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SWITCH_PROGRAM,
        symbol: "Sys80_5E_SwitchThread",
        parameters: SWITCH_PROGRAM_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::SwitchCoroutine,
        notes: "sub_4892B0 stores the thread id and returns 3; the scheduler (sub_48CD70) runs that thread next in the same pass and continues from its successor, and an unknown id (0 included) ends the pass.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_YIELD,
        symbol: "Sys80_5F_Yield",
        parameters: NO_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Yield,
        notes: "Target sub_4892D0 returns native interpreter status 1 without BP stack or CThread state changes. The outer interpreter ends the current cooperative pass and resumes at the next instruction.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SET_MAIN_LOOP_WAIT_OVERRIDE,
        symbol: "Sys80_52_SetMainLoopWaitOverride",
        parameters: MAIN_LOOP_WAIT_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target handler 0x00489050 leaves the popped i32 in EAX and sub_48D1B0 stores it in dword_503EF8; sub_48D1C0 is consumed by the executable main-loop pacing branch.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_ARM_NEXT_BINARY_ASYNC,
        symbol: "Sys80_53_ArmNextBinaryAsync",
        parameters: NO_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target handler 0x00489070 stores one in dword_5668A0. Sys81:30 and Graph90:F6 consume and reset this one-shot flag when selecting DCProcReadBinary or DCProcDecodeBMV.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SET_EXCLUSIVE_THREAD,
        symbol: "Sys80_5D_SetExclusiveThread",
        parameters: EXCLUSIVE_THREAD_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target handler 0x00489290 stores either the current CThread pointer or null through sub_48D170 and mirrors whether exclusive scheduling is enabled.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_CONFIGURE_DISPLAY_MODE,
        symbol: "Sys80_60_ConfigureDisplayMode",
        parameters: DISPLAY_MODE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target handler 0x004892E0 pops fullscreen, adapter, then mode and calls sub_461110(mode, adapter, fullscreen); adapter>=2 or mode>=8 is fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_QUERY_FULLSCREEN,
        symbol: "Sys80_61_QueryFullscreen",
        parameters: NO_PARAMETERS,
        returns: "current fullscreen flag",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target handler 0x00489340 pushes sub_45F640(), the current fullscreen state.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_CONFIGURE_FULLSCREEN_HOTKEYS,
        symbol: "Sys80_62_ConfigureFullscreenHotkeys",
        parameters: FULLSCREEN_HOTKEY_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target handler 0x00489360 passes an enable flag and zero-terminated descriptor list to sub_461740. The list may contain at most fifteen nonzero entries.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SET_ASPECT_PRESERVING_SCALING,
        symbol: "Sys80_63_SetAspectPreservingScaling",
        parameters: BOOL_VALUE_PARAMETER,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target handler 0x004893A0 calls sub_45E860(argument==0): zero selects stretch mode 1; nonzero selects aspect-preserving fit mode 0.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SET_WINDOW_VISIBLE,
        symbol: "Sys80_64_SetWindowVisible",
        parameters: BOOL_VALUE_PARAMETER,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target handler 0x004893C0 calls sub_49A170 and then clears transient input state through sub_46DA20.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_MINIMIZE_WINDOW,
        symbol: "Sys80_65_MinimizeWindow",
        parameters: NO_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_489ED0: CloseWindow(hWndParent), clears the transient input records (sub_46DF80) and sets the minimize latch dword_566708.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SET_WINDOW_TITLE,
        symbol: "Sys80_66_SetWindowTitle",
        parameters: STRING_VALUE_PARAMETER,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target handler 0x00489400 calls SetWindowTextA and persists the title in the engine's 256-byte title buffer on success.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SET_CURSOR_INDEX,
        symbol: "Sys80_67_SetCursorIndex",
        parameters: CURSOR_INDEX_PARAMETER,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target handler 0x00489430 validates 0..=4 and stores the index used by WM_SETCURSOR.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SET_NATIVE_CLOSE_MODE,
        symbol: "Sys80_68_SetNativeCloseMode",
        parameters: BOOL_VALUE_PARAMETER,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target sub_49A0D0 stores the value and updates SC_CLOSE. Zero intercepts WM_CLOSE into an engine event; nonzero permits native close processing.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_REQUEST_WINDOW_CLOSE,
        symbol: "Sys80_69_RequestWindowClose",
        parameters: NO_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target handler 0x004894B0 calls PostMessageA(hWndParent, WM_CLOSE, 0, 0). Portable frontends defer the host close request until the current scheduler pass returns, then apply the Sys80:68 native-close-mode gate so intercepted closes enter sysmsg._bp as [2,0,0] instead of quitting immediately.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_TERMINATE_INTERPRETER,
        symbol: "Sys80_6A_TerminateInterpreter",
        parameters: NO_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::TerminateInterpreter,
        notes: "Target handler returns native interpreter status 6. The outer runtime treats this as interpreter termination; it is not a resumable cooperative yield.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SELECT_BOOTSTRAP,
        symbol: "Sys80_6B_SelectBootstrap",
        parameters: BOOTSTRAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::SwitchCoroutine,
        notes: "Target handler 0x004894E0 configures the primary root/archive pair through sub_4650F0 and returns interpreter status 5 to re-enter bootstrap selection.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SET_FILE_DROP_ENABLED,
        symbol: "Sys80_6C_SetFileDropEnabled",
        parameters: BOOL_VALUE_PARAMETER,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target sub_463A80 stores the flag, calls DragAcceptFiles and clears the pending dropped-file buffer.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_QUERY_DROPPED_FILE,
        symbol: "Sys80_6D_QueryDroppedFile",
        parameters: OUTPUT_STRING_PARAMETER,
        returns: "boolean availability",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target sub_463AE0 copies the pending dropped path and returns one when enabled and nonempty. Querying does not clear the native buffer.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SET_RASTER_WAIT_ENABLED,
        symbol: "Sys80_6E_SetRasterWaitEnabled",
        parameters: BOOL_VALUE_PARAMETER,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target handler 0x00489560 stores the raw value in dword_507220, which gates the raster/vblank wait path.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_QUERY_DISPLAY_ASPECT_MISMATCH,
        symbol: "Sys80_6F_QueryDisplayAspectMismatch",
        parameters: NO_PARAMETERS,
        returns: "boolean aspect mismatch",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target sub_461550 compares the desktop aspect ratio with the configured game/base aspect after scaling both by 10000.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_ALLOCATE_GLOBAL_CONFIG,
        symbol: "Sys80_70_AllocateGlobalConfig",
        parameters: GLOBAL_CONFIG_SHIFT_PARAMETER,
        returns: "boolean allocation success",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target handler validates shift<=12, allocates 4096<<shift bytes, frees the previous buffer and zero-fills the replacement.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_CLEAR_GLOBAL_CONFIG,
        symbol: "Sys80_71_ClearGlobalConfig",
        parameters: NO_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target handler zeroes exactly the currently allocated global-config byte count.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SET_SAVE_DATA_INTEGRITY,
        symbol: "Sys80_74_SetSaveDataIntegrity",
        parameters: BOOL_VALUE_PARAMETER,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target handler stores the raw integer in dword_506BDC. Zero alone disables save-slot encryption/checking.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SAVE_CONFIG_SLOT,
        symbol: "Sys80_78_SaveConfigSlot",
        parameters: SAVE_SLOT_WRITE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target writes BGI%04d.cad as a 64-byte SYSTEMTIME/label header followed by the whole global-config buffer. Labels of 40 bytes or more are fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_LOAD_CONFIG_SLOT,
        symbol: "Sys80_79_LoadConfigSlot",
        parameters: SAVE_SLOT_PARAMETERS,
        returns: "status 0..=3",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target loads and optionally decrypts the slot, applies its payload, then restores the first 1024 global-config bytes. Status: 0 success, 1 missing/empty, 2 wrong length, 3 integrity failure.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_READ_CONFIG_SLOT_HEADER,
        symbol: "Sys80_7A_ReadConfigSlotHeader",
        parameters: SAVE_SLOT_HEADER_PARAMETERS,
        returns: "status 0..=2",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target copies the first 64 bytes only when the file length equals 64 plus the current global-config size. Status: 0 success, 1 missing/empty, 2 partial or wrong length.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_VALIDATE_CONFIG_SLOT,
        symbol: "Sys80_7B_ValidateConfigSlot",
        parameters: SAVE_SLOT_PARAMETERS,
        returns: "status 0..=3",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target reads and optionally verifies/decrypts the complete slot without applying the payload. Status codes match Sys80_79.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_LOAD_GLOBAL_USER_DATA,
        symbol: "Sys80_80_LoadGlobalUserData",
        parameters: NO_PARAMETERS,
        returns: "window_x, window_y, status",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target loads BGI.gdb, decodes SDC data and maps missing/decode failures to status 1/2.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SAVE_GLOBAL_USER_DATA,
        symbol: "Sys80_81_SaveGlobalUserData",
        parameters: NO_PARAMETERS,
        returns: "boolean success",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target writes the BURIKO GDB 3.00 header, global data, resource names and read-flag tables to BGI.gdb.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_WRITE_GLOBAL_DATA_BLOCK,
        symbol: "Sys80_82_WriteGlobalDataBlock",
        parameters: GLOBAL_DATA_BLOCK_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target copies into the fixed 1 MiB global data region and treats zero/out-of-range transfers as fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_READ_GLOBAL_DATA_BLOCK,
        symbol: "Sys80_83_ReadGlobalDataBlock",
        parameters: GLOBAL_DATA_BLOCK_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target copies from the fixed 1 MiB global data region and treats zero/out-of-range transfers as fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_RESET_STRUCTURED_HISTORY,
        symbol: "Sys80_90_ResetStructuredHistory",
        parameters: STRUCTURED_HISTORY_CAPACITY_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Frees all native 0x40-byte history nodes, clears head/tail/count and stores the new capacity.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_STRUCTURED_HISTORY_COUNT,
        symbol: "Sys80_91_StructuredHistoryCount",
        parameters: NO_PARAMETERS,
        returns: "i32 record count",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Returns the process-global bounded history count.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_APPEND_STRUCTURED_HISTORY_FIELDS,
        symbol: "Sys80_94_AppendStructuredHistoryFields",
        parameters: STRUCTURED_HISTORY_FIELDS_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Allocates a native history node, validates 31/255-byte string limits, appends at the tail and evicts the oldest record at capacity.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_READ_STRUCTURED_HISTORY,
        symbol: "Sys80_95_ReadStructuredHistory",
        parameters: STRUCTURED_HISTORY_READ_PARAMETERS,
        returns: "boolean success (invalid index is fatal)",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Index zero selects the newest record; writes the target record layout without the extended field.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_APPEND_STRUCTURED_HISTORY_RECORD,
        symbol: "Sys80_96_AppendStructuredHistoryRecord",
        parameters: STRUCTURED_HISTORY_RECORD_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Imports the target scalar/string record layout, including the optional extended text at byte 512.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_READ_STRUCTURED_HISTORY_EXTENDED,
        symbol: "Sys80_97_ReadStructuredHistoryExtended",
        parameters: STRUCTURED_HISTORY_READ_PARAMETERS,
        returns: "boolean success (invalid index is fatal)",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Same newest-first lookup as 0x95 and additionally copies the optional extended text.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_INDEXED_RECORD_OPEN,
        symbol: "Sys80_98_IndexedRecordOpen",
        parameters: INDEXED_RECORD_OPEN_PARAMETERS,
        returns: "status 0 or 2",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Creates a capacity-bounded fixed-record table and writes a monotonically allocated integer identifier.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_INDEXED_RECORD_CLOSE,
        symbol: "Sys80_99_IndexedRecordClose",
        parameters: INDEXED_RECORD_HANDLE_PARAMETERS,
        returns: "status 0 or 1",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Closes an indexed-record table; missing identifiers return one.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_INDEXED_RECORD_COUNT,
        symbol: "Sys80_9A_IndexedRecordCount",
        parameters: INDEXED_RECORD_COUNT_PARAMETERS,
        returns: "status 0 or 1",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Writes the current record count through the output pointer.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_INDEXED_RECORD_PUSH,
        symbol: "Sys80_9C_IndexedRecordPush",
        parameters: INDEXED_RECORD_PUSH_PARAMETERS,
        returns: "status 0 or 1",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Copies one fixed-size record to the front and truncates records beyond capacity.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_INDEXED_RECORD_LOAD,
        symbol: "Sys80_9D_IndexedRecordLoad",
        parameters: INDEXED_RECORD_LOAD_PARAMETERS,
        returns: "status 0..=2",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Loads one newest-first record; missing table is one and invalid index is two.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_INDEXED_RECORD_REMOVE,
        symbol: "Sys80_9E_IndexedRecordRemove",
        parameters: INDEXED_RECORD_REMOVE_PARAMETERS,
        returns: "status 0..=2",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Removes up to count records starting at a newest-first index.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_POLL_QUEUED_EVENT,
        symbol: "Sys80_A0_PollQueuedEvent",
        parameters: QUEUED_EVENT_POLL_PARAMETERS,
        returns: "boolean dequeued",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Pops the oldest process-global three-DWORD event into caller memory.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_POST_QUEUED_EVENT,
        symbol: "Sys80_A1_PostQueuedEvent",
        parameters: QUEUED_EVENT_POST_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Queues [0,event_code,parameter] in the target critical-section protected FIFO.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SET_REGISTERED_OBJECT_STATE,
        symbol: "Sys80_A8_SetRegisteredObjectState",
        parameters: REGISTERED_OBJECT_STATE_SET_PARAMETERS,
        returns: "boolean object found",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48A1B0: writes DCIndProc+0x10 (the enabled flag, 1 at construction) of the registered processor and pushes whether the handle exists. Processors with 0 are skipped by the per-frame update and drain no messages.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_GET_REGISTERED_OBJECT_STATE,
        symbol: "Sys80_A9_GetRegisteredObjectState",
        parameters: REGISTERED_OBJECT_STATE_GET_PARAMETERS,
        returns: "boolean object found",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48A200: pops the output pointer then the handle, writes DCIndProc+0x10 and pushes whether the handle exists.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_QUEUE_REGISTERED_OBJECT_MESSAGE,
        symbol: "Sys80_AC_QueueRegisteredObjectMessage",
        parameters: REGISTERED_OBJECT_MESSAGE_PARAMETERS,
        returns: "boolean object found",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_48A250: queues a 1..256-word message on the registered processor (other counts queue nothing) and pushes whether the handle exists. Each frame enabled processors drain their queue newest-registration first: [0, v] sets the enabled flag, other messages go to sub_44A250 (activate, select group/item, pointer processing, item bitmaps, item depth/position).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SET_SYSTEM_MODE_FLAG,
        symbol: "Sys80_AF_SetSystemModeFlag",
        parameters: SYSTEM_MODE_PARAMETERS,
        returns: "boolean accepted",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Accepts only zero or one and stores the process-global mode queried by the System81 extension.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_CREATE_EXCLUSION_SECTION,
        symbol: "Sys80_B0_CreateExclusionSection",
        parameters: EXCLUSION_CREATE_PARAMETERS,
        returns: "boolean created",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Creates a named capacity-limited CExclusion section; duplicate names return zero.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_DELETE_EXCLUSION_SECTION,
        symbol: "Sys80_B1_DeleteExclusionSection",
        parameters: EXCLUSION_NAME_PARAMETERS,
        returns: "native status",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Returns 0x80000001 when missing and 0x80000002 while holders or waiters remain.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_WAIT_EXCLUSION_SECTION,
        symbol: "Sys80_B4_WaitExclusionSection",
        parameters: EXCLUSION_PRIORITY_PARAMETERS,
        returns: "deferred native status",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "Installs CProcExclusion, queues the current thread by descending priority and resumes only when it is queue head and capacity is available.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_LEAVE_EXCLUSION_SECTION,
        symbol: "Sys80_B5_LeaveExclusionSection",
        parameters: EXCLUSION_NAME_PARAMETERS,
        returns: "native status",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Releases one holder entry for the current thread; missing/not-owner map to 0x80000001/0x80000003.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_QUERY_EXCLUSION_SECTION,
        symbol: "Sys80_B6_QueryExclusionSection",
        parameters: EXCLUSION_PRIORITY_PARAMETERS,
        returns: "native status",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Returns zero when capacity covers current holders plus all queued priorities greater than or equal to the request, otherwise 0x80000004.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_INTERN_RESOURCE_NAME,
        symbol: "Sys80_84_InternResourceName",
        parameters: RESOURCE_NAME_PARAMETERS,
        returns: "constant success value 1",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target handler 0x004899B0 calls sub_46B430, which interns the name and then returns one. The table's stable index remains internal and is not returned to BP code.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_RESOURCE_NAME_EXISTS,
        symbol: "Sys80_85_ResourceNameExists",
        parameters: RESOURCE_NAME_PARAMETERS,
        returns: "boolean existence",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target handler 0x004899E0 performs lookup only and must not insert missing names.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_CREATE_OR_RESIZE_READ_FLAG_TABLE,
        symbol: "Sys80_88_CreateOrResizeReadFlagTable",
        parameters: READ_FLAG_TABLE_PARAMETERS,
        returns: "status/success value",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target handler 0x00489A10 calls ReadFlagTable_CreateOrResize. It is not a scenario-code preprocessing syscall.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SET_READ_FLAG_BIT,
        symbol: "Sys80_89_SetReadFlagBit",
        parameters: READ_FLAG_BIT_SET_PARAMETERS,
        returns: "mapped native status",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target handler 0x00489A40 changes one bit in a named compact bitset.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SET_READ_FLAG_RANGE,
        symbol: "Sys80_8A_SetReadFlagRange",
        parameters: READ_FLAG_RANGE_PARAMETERS,
        returns: "mapped native status",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target handler 0x00489AC0 calls ReadFlagTable_SetRange at 0x00446D40 and validates the complete range.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_QUERY_READ_FLAG_BIT,
        symbol: "Sys80_8B_QueryReadFlagBit",
        parameters: READ_FLAG_QUERY_PARAMETERS,
        returns: "mapped native status; queried boolean is written to output_ptr",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target handler 0x00489B80 writes the queried bit through a BP pointer and returns a separate status.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_ENCODE_DATA,
        symbol: "Sys80_C0_EncodeData",
        parameters: SYS_ENCODE_DATA_PARAMETERS,
        returns: "encoded byte length through procedure completion",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "Target handler constructs CProcEncodeData from destination, source, and length; completion publishes the encoded length.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_DECODE_SDC_BUFFER,
        symbol: "Sys80_C1_DecodeSdcBuffer",
        parameters: SYS_DECODE_SDC_BUFFER_PARAMETERS,
        returns: "decoded byte length",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Exact 0x0048A4D0 saves the first native pop in ESI and passes it in EAX as the SDC source to sub_4938F0; the second pop is its destination argument.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_ENCODE_STRUCT_ARRAY,
        symbol: "Sys80_C4_EncodeStructArray",
        parameters: SYS_ENCODE_STRUCT_ARRAY_PARAMETERS,
        returns: "encoded byte length through procedure completion",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "Target handler constructs CProcEncodeStruct, applying DCFS record transform and SDC compression asynchronously.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_DECODE_STRUCT_ARRAY,
        symbol: "Sys80_C5_DecodeStructArray",
        parameters: SYS_DECODE_STRUCT_ARRAY_PARAMETERS,
        returns: "decoded record count",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target handler synchronously performs SDC decompression followed by DCFS reconstruction and returns the record count.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_DECODE_DATA,
        symbol: "Sys80_CF_DecodeData",
        parameters: SYS_DECODE_DATA_PARAMETERS,
        returns: "decoded length through procedure completion",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "BGI 1.622 499120/465880: SDC, DSC, native CBG v1/v2 buffer or raw copy; input length is not decoded capacity. Tagged BP addresses are resolved; invalid decode returns zero.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_RECORD_TABLE_OPEN,
        symbol: "Sys80_D0_RecordTableOpen",
        parameters: SYS_RECORD_TABLE_OPEN_PARAMETERS,
        returns: "0 or 0x80000001",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target allocates an integer-handle keyed table whose entries are separately allocated record nodes; there is no fixed table capacity. Record sizes <=1 return 0x80000001.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_RECORD_TABLE_CLOSE,
        symbol: "Sys80_D1_RecordTableClose",
        parameters: SYS_RECORD_TABLE_CLOSE_PARAMETERS,
        returns: "0 or 0x80000002",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target removes the integer-handle table; a missing table returns 0x80000002.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_RECORD_TABLE_INSERT,
        symbol: "Sys80_D2_RecordTableInsert",
        parameters: SYS_RECORD_TABLE_INSERT_PARAMETERS,
        returns: "0 or 0x80000002",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target inserts a separately allocated fixed-size record node or replaces the existing node selected by string key.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_RECORD_TABLE_REMOVE,
        symbol: "Sys80_D3_RecordTableRemove",
        parameters: SYS_RECORD_TABLE_REMOVE_PARAMETERS,
        returns: "0, 0x80000002, or 0x80000003",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target distinguishes missing table from missing key.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_RECORD_TABLE_FETCH,
        symbol: "Sys80_D4_RecordTableFetch",
        parameters: SYS_RECORD_TABLE_FETCH_PARAMETERS,
        returns: "0, 0x80000002, or 0x80000003",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target fetches by key or linked-list insertion-order index and copies the fixed-size record.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_CLEAR_STRING_NAMESPACES,
        symbol: "Sys80_D8_ClearStringNamespaces",
        parameters: NO_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target clears all string namespaces except protected table id 0x80000000.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_STRING_NAMESPACE_COUNT,
        symbol: "Sys80_D9_StringNamespaceCount",
        parameters: SYS_STRING_NAMESPACE_COUNT_PARAMETERS,
        returns: "entry count, or zero when absent",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target returns the number of interned strings in one namespace.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_REPLACE_STRING_NAMESPACE,
        symbol: "Sys80_DA_ReplaceStringNamespace",
        parameters: SYS_REPLACE_STRING_NAMESPACE_PARAMETERS,
        returns: "boolean success",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target replaces a namespace from a packed string sequence or removes it when count is zero.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SERIALIZE_STRING_NAMESPACE,
        symbol: "Sys80_DB_SerializeStringNamespace",
        parameters: SYS_SERIALIZE_STRING_NAMESPACE_PARAMETERS,
        returns: "required byte count",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target serializes one namespace including each NUL terminator.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_INTERN_STRING_NAMESPACE,
        symbol: "Sys80_DC_InternStringNamespace",
        parameters: SYS_INTERN_STRING_NAMESPACE_PARAMETERS,
        returns: "zero-based string index",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target interns/appends a string in one namespace with no graph-resource side effect.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_STRING_NAMESPACE_ENTRY_LENGTH,
        symbol: "Sys80_DE_StringNamespaceEntryLength",
        parameters: SYS_STRING_NAMESPACE_ENTRY_LENGTH_PARAMETERS,
        returns: "0, 0x80000001, or 0x80000002",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target reports the encoded byte length of one namespace entry without copying it.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_LAUNCH_PROCESS_WAIT,
        symbol: "Sys80_E0_LaunchProcessWait",
        parameters: SYS_LAUNCH_PROCESS_WAIT_PARAMETERS,
        returns: "boolean launch success",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target launches a process, waits for it, and optionally restores the parent window.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_RESTART_WITH_COMMAND,
        symbol: "Sys80_E1_RestartWithCommand",
        parameters: SYS_RESTART_WITH_COMMAND_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::TerminateInterpreter,
        notes: "Target accepts nullable working/error pointers, requires a non-null command pointer, stores restart fields, destroys the main window, returns native control status 6, and launches after WinMain cleanup.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_LAUNCH_PROCESS_WAIT_UNINSTALLER,
        symbol: "Sys80_E2_LaunchProcessWaitUninstaller",
        parameters: SYS_LAUNCH_PROCESS_WAIT_UNINSTALLER_PARAMETERS,
        returns: "boolean launch success",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target launches and waits, then additionally waits for the named uninstaller synchronization object.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SHELL_OPEN,
        symbol: "Sys80_E3_ShellOpen",
        parameters: SYS_SHELL_OPEN_PARAMETERS,
        returns: "boolean success",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target invokes ShellExecute with the open verb.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_WRITE_GAME_ID,
        symbol: "Sys80_E8_WriteGameId",
        parameters: SYS_WRITE_GAME_ID_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target writes the literal Tayutama2TV string.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_HASH_FILE,
        symbol: "Sys80_E9_HashFile",
        parameters: SYS_HASH_FILE_PARAMETERS,
        returns: "boolean read success",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target hashes bytes with h=byte+233*h and four rolling tail accumulators.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SET_UNINSTALLER_PRODUCT,
        symbol: "Sys80_EA_SetUninstallerProduct",
        parameters: SYS_SET_UNINSTALLER_PRODUCT_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target formats and stores the mutex name `Uninstaller for <product> is executing.` used by the uninstaller wait loop.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SHOW_INPUT_DIALOG,
        symbol: "Sys80_F0_ShowInputDialog",
        parameters: SYS_SHOW_INPUT_DIALOG_PARAMETERS,
        returns: "dialog result; zero on cancel",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target sub_471FB0 edits one Shift-JIS buffer plus two DWORD options. VM mediation now preserves all three output writes while the host supplies a cross-platform modal editor.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_SHOW_INSTALLER_DIALOG,
        symbol: "Sys80_F1_ShowInstallerDialog",
        parameters: SYS_SHOW_INSTALLER_DIALOG_PARAMETERS,
        returns: "dialog result; zero on cancel",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target sub_472050 opens one of two installer dialog resources using four text fields and two mode globals. Portable hosts expose the same modal success/cancel boundary.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_RUN_INSTALLER_WORKFLOW,
        symbol: "Sys80_F2_RunInstallerWorkflow",
        parameters: SYS_RUN_INSTALLER_WORKFLOW_PARAMETERS,
        returns: "boolean success",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target sub_472150 creates the installation tree, copies parallel file lists, writes uninst.lst and InstalledFolder metadata. The portable host performs those operations without Win32-only dialog/resource internals.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_RUN_SHORTCUT_INSTALLER_WORKFLOW,
        symbol: "Sys80_F3_RunShortcutInstallerWorkflow",
        parameters: SYS_RUN_SHORTCUT_INSTALLER_WORKFLOW_PARAMETERS,
        returns: "boolean success",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target sub_471530 creates the primary desktop shortcut and up to two program-menu shortcuts from one target root and two relative targets. Portable hosts preserve the same branching and file destinations.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_REMOVE_UNINSTALL_LISTED_FILES,
        symbol: "Sys80_F4_RemoveUninstallListedFiles",
        parameters: SYS_REMOVE_UNINSTALL_LISTED_FILES_PARAMETERS,
        returns: "boolean list-open success",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target reads raw LF-separated Shift-JIS entries from root\\uninst.lst and deletes each except leading-@ directives, exact `uninst.lst`, and byte-exact exclusions.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_APPEND_UNINSTALL_LIST_ENTRIES,
        symbol: "Sys80_F5_AppendUninstallListEntries",
        parameters: SYS_APPEND_UNINSTALL_LIST_ENTRIES_PARAMETERS,
        returns: "boolean success",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target requires root\\uninst.lst to exist, appends entries absent by C-string substring search, and rewrites the terminal NUL.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_REMOVE_INSTALLER_SHORTCUTS,
        symbol: "Sys80_F6_RemoveInstallerShortcuts",
        parameters: SYS_REMOVE_INSTALLER_SHORTCUTS_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target removes desktop/program-menu shortcuts and optionally the group.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_CREATE_SHORTCUT,
        symbol: "Sys80_F7_CreateShortcut",
        parameters: SYS_CREATE_SHORTCUT_PARAMETERS,
        returns: "boolean success",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target passes ECX=0 to sub_4713F0: null group writes the desktop, non-null group writes CSIDL_PROGRAMS\\group. Hosts create .lnk on Windows and executable desktop-compatible launchers elsewhere.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_READ_INSTALLED_FOLDER,
        symbol: "Sys80_F8_ReadInstalledFolder",
        parameters: SYS_READ_INSTALLED_FOLDER_PARAMETERS,
        returns: "boolean success",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target reads HKLM Software\\vendor\\product InstalledFolder. Windows reads the native registry and all hosts fall back to the persistent platform store.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_DELETE_INSTALLED_REGISTRY_KEY,
        symbol: "Sys80_F9_DeleteInstalledRegistryKey",
        parameters: SYS_DELETE_INSTALLED_REGISTRY_KEY_PARAMETERS,
        returns: "boolean success",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target deletes HKLM Software\\vendor\\product.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_READ_WINDOWS_PATH_FILE,
        symbol: "Sys80_FA_ReadWindowsPathFile",
        parameters: SYS_READ_WINDOWS_PATH_FILE_PARAMETERS,
        returns: "boolean success",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target reads WindowsDir\\file and accepts only NUL-terminated content ending in a backslash before the terminator.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_WRITE_WINDOWS_DIRECTORY,
        symbol: "Sys80_FB_WriteWindowsDirectory",
        parameters: SYS_WRITE_WINDOWS_DIRECTORY_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target calls sub_467570 with selector zero and writes the resulting directory. The VM writes the host special-folder result through the exact BP pointer contract.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_REGISTER_FILE_ASSOCIATION,
        symbol: "Sys80_FC_RegisterFileAssociation",
        parameters: SYS_REGISTER_FILE_ASSOCIATION_PARAMETERS,
        returns: "boolean success",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target writes the extension/class/description/icon/open-command hierarchy under HKCR. Windows writes HKCU\\Software\\Classes; all hosts persist the equivalent association in the platform store.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_QUERY_LAUNCHER_MODE,
        symbol: "Sys80_FD_QueryLauncherMode",
        parameters: NO_PARAMETERS,
        returns: "target launcher-mode global",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target returns the launcher-mode global established by startup. The portable runtime returns the configured launcher mode through SysApi instead of forcing a constant fallback.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_INSTALLER_FEATURE_AVAILABLE,
        symbol: "Sys80_FE_InstallerFeatureAvailable",
        parameters: NO_PARAMETERS,
        returns: "constant one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target handler pushes constant 1.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SYS_COPY_INDEXED_NAMESPACE_RECORD,
        symbol: "Sys80_DD_CopyIndexedNamespaceEntry",
        parameters: COPY_INDEXED_NAMESPACE_RECORD_PARAMETERS,
        returns: "mapped status",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "Target handler 0x0048A900 passes destination/table/index to sub_495850. The helper locates the 12-byte metadata slot, then copies the entry payload pointer by its stored byte length; sub_495550 constructs those payloads as Shift-JIS strings including the terminating NUL. Missing table/index map to 0x80000001/0x80000002.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_REQUEST_REDRAW,
        symbol: "Graph90_00_RequestRedraw",
        parameters: GRAPH90_REQUEST_REDRAW_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_479EA0: sub_4622D0 (full) / sub_4622E0 (partial) redraw request (dword_566714/506BD0); a pending full request stays full.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_SCHEDULER_GATE,
        symbol: "Graph90_01_SetSchedulerGate",
        parameters: GRAPH90_SET_SCHEDULER_GATE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479ED0 -> sub_462260: dword_506BD4; the main loop (sub_462100) presents only while set, otherwise the last frame stays (take_present_due).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_FRAME_RATE,
        symbol: "Graph90_02_SetFrameRate",
        parameters: GRAPH90_SET_FRAME_RATE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479EF0 -> sub_462280: 1..=1000 else fatal (無効なフレームレート); interval 1000/rate (dword_506BCC), next presentation due at once.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_INITIALIZE_BITMAP_MEMORY_MANAGER,
        symbol: "Graph90_03_InitializeBitmapMemoryManager",
        parameters: GRAPH90_INITIALIZE_BITMAP_MEMORY_MANAGER_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479F60 -> sub_439C80: DCCache budget; more than 0x20000000 is fatal (無効なキャッシュサイズ).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CREATE_WORK_BITMAP,
        symbol: "Graph90_04_CreateWorkBitmap",
        parameters: GRAPH90_CREATE_WORK_BITMAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479FC0 -> sub_4432A0: bitmap < 0x4000 (sub_4986D0) created in the screen's size/format and filled from the current frame.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CREATE_PRIORITIZED_WORK_BITMAP,
        symbol: "Graph90_05_CreatePrioritizedWorkBitmap",
        parameters: GRAPH90_CREATE_PRIORITIZED_WORK_BITMAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479FF0 -> sub_443340: priority (sub_498720) then bitmap checks; the display tree up to the priority is rendered into it.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_CENTER,
        symbol: "Graph90_06_SetCenter",
        parameters: GRAPH90_SET_CENTER_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47A030 -> sub_443230: screen centre +0x40/+0x44 = second/first argument; screen marked dirty.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_SYNC_LOAD_HOLD,
        symbol: "Graph90_07_SetSyncLoadHold",
        parameters: GRAPH90_SET_SYNC_LOAD_HOLD_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47A060 -> sub_402070: synchronous-load hold dword_565AF0, open window dword_565AF4 cleared (sub_402080).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_DISPLAY_ENABLED,
        symbol: "Graph90_08_SetDisplayEnabled",
        parameters: GRAPH90_SET_DISPLAY_ENABLED_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47A080 -> sub_41A9B0: stores dword_5076AC (written only) and marks the screen dirty.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_DEFAULT_PRIORITY,
        symbol: "Graph90_09_SetDefaultPriority",
        parameters: GRAPH90_SET_DEFAULT_PRIORITY_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47A0A0 -> sub_462400: priority check, CObjectManager+0x48 = priority << 16 for objects created later.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_OBJECT_UPDATE_REDRAW_POLICY,
        symbol: "Graph90_0A_SetObjectUpdateRedrawPolicy",
        parameters: GRAPH90_SET_OBJECT_UPDATE_REDRAW_POLICY_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47A0D0 -> sub_431DB0: dword_50768C (procedure redraw requests) and dword_565B88 (full).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_CURRENT_BITMAP,
        symbol: "Graph90_0B_SetCurrentBitmap",
        parameters: GRAPH90_SET_CURRENT_BITMAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47A100 -> sub_407BA0: bitmap manager current slot.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_DISPLAY_OPTIONS,
        symbol: "Graph90_0C_SetDisplayOptions",
        parameters: GRAPH90_SET_DISPLAY_OPTIONS_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47A120 -> sub_440930: p0 enables CDspObjWindow's own-buffer blit (dword_565B54, .bss 0), p1 (sub_498920, <= 256) is the global transparency dword_565B58; the sixteen windows are invalidated. sub_42B520 draws the window buffer only while enabled, at 256 - ((256 - p1) * (256 - own) >> 8).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_RASTER_FORMAT_MODE,
        symbol: "Graph90_0D_SetRasterFormatMode",
        parameters: GRAPH90_SET_RASTER_FORMAT_MODE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47A160 -> sub_42E050: anti-aliasing level, fatal above 3 (signed); hardware layouts for 0..=3, none otherwise; fonts rebuilt.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_REGISTER_FONT,
        symbol: "Graph90_0E_RegisterFont",
        parameters: GRAPH90_REGISTER_FONT_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47A1C0 -> sub_462340 -> sub_42F340: cache count, italic, width, height, face; unknown face / height / width / cache < 2 are fatal with the 1.622 texts.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_BITMAP_UNBLEND_COLOR,
        symbol: "Graph90_0F_SetBitmapUnblendColor",
        parameters: GRAPH90_SET_BITMAP_UNBLEND_COLOR_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47A2C0 -> sub_407B40: dword_565B28, applied by sub_408E80 to format-2 pixels of every later bitmap load.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_LOAD_BITMAP,
        symbol: "Graph90_10_LoadBitmap",
        parameters: GRAPH90_LOAD_BITMAP_PARAMETERS,
        returns: "procedure completion",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "sub_47A2E0: pops name, archive, bitmap (checked); cached data or a synchronous load (no '/', sub_402080) returns 0, otherwise CProcLoadBitmap (returns 2); a missing file is the media-prompt error.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CREATE_BITMAP,
        symbol: "Graph90_11_CreateBitmap",
        parameters: GRAPH90_CREATE_BITMAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_47A4B0 -> sub_4026C0 -> sub_407D90: pops format, height, width, bitmap; a failed creation is fatal (無効なピクセルモード).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_RELEASE_BITMAP,
        symbol: "Graph90_12_ReleaseBitmap",
        parameters: GRAPH90_RELEASE_BITMAP_PARAMETERS,
        returns: "boolean success",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47A550 -> sub_407CE0: releases the slot, 1 when it existed.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_FILL_BITMAP,
        symbol: "Graph90_13_FillBitmap",
        parameters: GRAPH90_FILL_BITMAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47A580 -> sub_408030: bitmap check, unregistered bitmap fatal; 0 clears all bytes, otherwise per format (RGB555, low 24 bits, all 32 bits, low byte; 4..6 untouched).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CREATE_BITMAP_FROM_PIXELS,
        symbol: "Graph90_14_CreateBitmapFromPixels",
        parameters: GRAPH90_CREATE_BITMAP_FROM_PIXELS_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47A5C0 -> sub_4080A0: pops pixels, format, height, width, bitmap (checked); packed 3-byte format 1, otherwise raw; unblend colour applied.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_COPY_BITMAP_PIXELS,
        symbol: "Graph90_15_CopyBitmapPixels",
        parameters: GRAPH90_COPY_BITMAP_PIXELS_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47A630 -> sub_4081F0: pops bitmap, capacity, written, buffer; {2,3,4,1}[format] bytes per pixel; missing bitmap / small buffer fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_QUERY_BITMAP_INFO,
        symbol: "Graph90_16_QueryBitmapInfo",
        parameters: GRAPH90_QUERY_BITMAP_INFO_PARAMETERS,
        returns: "boolean found",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47A700 -> sub_407F10: six DWORDs (pointer cleared, pitch, width, height, format, bpp) and found.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_VALIDATE_BITMAP_FORMAT,
        symbol: "Graph90_17_ValidateBitmapFormat",
        parameters: GRAPH90_VALIDATE_BITMAP_FORMAT_PARAMETERS,
        returns: "status 0/1/2",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47A740 -> sub_4081A0: 0 same format, 0 after reducing 2 to 1, 1 missing, 2 otherwise.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_BLIT_BITMAP,
        symbol: "Graph90_18_BlitBitmap",
        parameters: GRAPH90_BLIT_BITMAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47A770 -> sub_402720: bitmaps, mode list (sub_4987B0) and level checks; missing/incompatible fatal; no overlap draws nothing before the format check.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SYNTHESIZE_BITMAP,
        symbol: "Graph90_19_SynthesizeBitmap",
        parameters: GRAPH90_SYNTHESIZE_BITMAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47A8A0 -> sub_4027E0: synthesize with optional grey-scale mask; 1.622 error texts.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_COMPOSITE_BITMAPS,
        symbol: "Graph90_1A_CompositeBitmaps",
        parameters: GRAPH90_COMPOSITE_BITMAPS_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47AA20 -> sub_402970/sub_412AB0: vector-map warp; checks in target order (maps, level, map validity, formats).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_COPY_BITMAP,
        symbol: "Graph90_1B_CopyBitmap",
        parameters: GRAPH90_COPY_BITMAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47ABC0 -> sub_402AC0/sub_413D90: gradation copy; size/format, type, level errors.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SCALE_BITMAP_REGION,
        symbol: "Graph90_1C_ScaleBitmapRegion",
        parameters: GRAPH90_SCALE_BITMAP_REGION_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47ACE0 -> sub_402C90/sub_414DF0: scaled region; output/reference range and format errors.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_TRANSFORM_BITMAP,
        symbol: "Graph90_1D_TransformBitmap",
        parameters: GRAPH90_TRANSFORM_BITMAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47AE90 -> sub_402EA0/sub_414AF0: rotate/scale; compatibility and zero-scale errors.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_BLIT_BITMAP_REGION,
        symbol: "Graph90_1E_BlitBitmapRegion",
        parameters: GRAPH90_BLIT_BITMAP_REGION_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47AF90 -> sub_4032C0: pops height, width, source y/x, source, y, x, destination; missing bitmaps fatal; clipped raw copy, zero size does nothing.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CREATE_BITMAP_REGION,
        symbol: "Graph90_1F_CreateBitmapRegion",
        parameters: GRAPH90_CREATE_BITMAP_REGION_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47B090 -> sub_4033A0: detached copy region; source (2), size (3), allocation (1) with the 1.622 texts.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_START_OBJECT_CONTROL,
        symbol: "Graph90_20_StartAlphaControl",
        parameters: GRAPH90_START_OBJECT_CONTROL_PARAMETERS,
        returns: "CProcCtrlDspObj completion",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "BGI 1.622 sub_47B1A0 -> sub_492640 (CProcCtrlDspObj): scope (sub_498720) and transparency (sub_498920) checks, zero rate and unknown object fatal; returns 2 (waits).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_START_NODE_CONTROL,
        symbol: "Graph90_21_StartMoveControl",
        parameters: GRAPH90_START_NODE_CONTROL_PARAMETERS,
        returns: "CProcCtrlDspObj completion",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "sub_47B390 -> sub_492640 with a position: same checks; x, y, curve, transparency, duration, rate, input.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_START_OBJECT_CONTROL_EX,
        symbol: "Graph90_22_StartAlphaControlEx",
        parameters: GRAPH90_START_OBJECT_CONTROL_EX_PARAMETERS,
        returns: "CProcCtrlDspObj completion",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "sub_47B290 -> sub_492640: transparency with rate numerator/denominator; same checks.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_START_NODE_CONTROL_EX,
        symbol: "Graph90_23_StartMoveControlEx",
        parameters: GRAPH90_START_NODE_CONTROL_EX_PARAMETERS,
        returns: "CProcCtrlDspObj completion",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "sub_47B4B0 -> sub_492640: position and transparency with rate numerator/denominator.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_START_SPECIAL_OBJECT_CONTROL,
        symbol: "Graph90_24_StartSpecialObjectControl",
        parameters: GRAPH90_START_SPECIAL_OBJECT_CONTROL_PARAMETERS,
        returns: "CProcCtrlDspObjSp completion",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "sub_47B5F0 -> sub_492740/sub_432D30: three-point path; unknown object, zero rate and the X-direction check (sub_495220) are fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_START_OBJECT_MOTION_CONTROL,
        symbol: "Graph90_28_StartObjectMotionControl",
        parameters: GRAPH90_START_OBJECT_MOTION_CONTROL_PARAMETERS,
        returns: "CProcCtrlDspObj completion",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "sub_47B780 -> sub_492860: scope, addition level (sub_4988C0) and transparency checks, then rate and object.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_START_SPLINE_OBJECT_CONTROL,
        symbol: "Graph90_29_StartSplineObjectControl",
        parameters: GRAPH90_START_SPLINE_OBJECT_CONTROL_PARAMETERS,
        returns: "CProcCtrlDspObjBC completion",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "sub_47B8F0 -> sub_492960/sub_432860: spline points; scope, addition, transparency, rate, object, point count with the 1.622 texts.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_START_SHAKE_OBJECT_CONTROL,
        symbol: "Graph90_2C_StartShakeObjectControl",
        parameters: GRAPH90_START_SHAKE_OBJECT_CONTROL_PARAMETERS,
        returns: "CProcShakeDspObj completion",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "sub_47BA80 -> sub_492A90/sub_43CC60 (CProcShakeDspObj): unknown object is ignored; pattern < 6, frequency, repeat count, rate >= frequency are fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_OBJECT_DRAW_ENABLED,
        symbol: "Graph90_30_SetObjectDrawEnabled",
        parameters: GRAPH90_SET_OBJECT_DRAW_ENABLED_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_47BBE0 -> sub_462470: draw enable through vtable+4; unknown object fatal (無効なオブジェクトハンドル).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_OBJECT_ENABLED,
        symbol: "Graph90_31_SetObjectEnabled",
        parameters: GRAPH90_SET_OBJECT_ENABLED_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47BC20 -> sub_462480: enable; unknown object fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_OBJECT_POSITION,
        symbol: "Graph90_33_SetObjectPosition",
        parameters: GRAPH90_SET_OBJECT_POSITION_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47BCB0 -> sub_4624B0: position; unknown object fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_OBJECT_MASK_ALPHA,
        symbol: "Graph90_34_SetObjectMaskAlpha",
        parameters: GRAPH90_SET_OBJECT_MASK_ALPHA_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47BD00 -> sub_4624E0: mask alpha; level check then object.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_OBJECT_FIXED_PARAMETER,
        symbol: "Graph90_35_SetObjectFixedParameter",
        parameters: GRAPH90_SET_OBJECT_FIXED_PARAMETER_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47BD50 -> sub_462500: fixed parameter; level check then object.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_OBJECT_SECONDARY_OFFSET,
        symbol: "Graph90_36_SetObjectSecondaryOffset",
        parameters: GRAPH90_SET_OBJECT_SECONDARY_OFFSET_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47BDA0 -> sub_462520: secondary offset; unknown object fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_OBJECT_PRIMARY_OFFSET,
        symbol: "Graph90_37_SetObjectPrimaryOffset",
        parameters: GRAPH90_SET_OBJECT_PRIMARY_OFFSET_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47BDF0 -> sub_462510: primary offset; unknown object fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_OBJECT_PROPERTY,
        symbol: "Graph90_38_SetObjectProperty",
        parameters: GRAPH90_SET_OBJECT_PROPERTY_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47BE40 -> sub_462570/sub_443C80: property setter; 5 unsupported and 254 bad arguments with the 1.622 texts, 255 unknown object; re-sorts on a key change.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_OBJECT_ALPHA_MULTIPLIER,
        symbol: "Graph90_39_SetObjectAlphaMultiplier",
        parameters: GRAPH90_SET_OBJECT_ALPHA_MULTIPLIER_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47BF10 -> sub_4624F0: alpha multiplier; level then object.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_OBJECT_PRIORITY,
        symbol: "Graph90_3A_SetObjectPriority",
        parameters: GRAPH90_SET_OBJECT_PRIORITY_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47BF60 -> sub_4625A0: priority check (sub_498720), 255 unknown object; re-inserted by priority.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_OBJECT_HIT_MASK_BITMAP,
        symbol: "Graph90_3C_SetObjectHitMaskBitmap",
        parameters: GRAPH90_SET_OBJECT_HIT_MASK_BITMAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47BFB0 -> sub_4625B0: hit mask; virtual object (1), invalid bitmap (2), unknown object (255).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_HIT_TEST_OBJECT_AT_POINTER,
        symbol: "Graph90_3D_HitTestObjectAtPointer",
        parameters: GRAPH90_HIT_TEST_OBJECT_AT_POINTER_PARAMETERS,
        returns: "hit-test value",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47C040 -> sub_4625D0: pointer hit value; unknown object fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_INVOKE_UNSUPPORTED_OBJECT_EXTENSION,
        symbol: "Graph90_3F_InvokeUnsupportedObjectExtension",
        parameters: GRAPH90_INVOKE_UNSUPPORTED_OBJECT_EXTENSION_PARAMETERS,
        returns: "target script error",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47C080 -> sub_462640: image cache extension unsupported by every class (3), unknown object (255).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH_SET_OBJECT_ALPHA,
        symbol: "Graph90_32_SetObjectAlpha",
        parameters: GRAPH_SET_OBJECT_ALPHA_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47BC60 -> sub_4624A0: transparency check (sub_498920), then object; recursive through members.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_CURRENT_OBJECT_BITMAP,
        symbol: "Graph90_40_SetCurrentObjectBitmap",
        parameters: GRAPH90_SET_CURRENT_OBJECT_BITMAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_47C0D0 -> sub_43DA40: current background as BackN with one bitmap; invalid bitmap fatal (背景には使用できません).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CONFIGURE_CURRENT_OBJECT_DUAL_BITMAP,
        symbol: "Graph90_41_ConfigureCurrentObjectDualBitmap",
        parameters: GRAPH90_CONFIGURE_CURRENT_OBJECT_DUAL_BITMAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47C130 -> sub_43DA70: BackB pair; transparency check first, alpha applied, then the pair (both named in the error).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CONFIGURE_CURRENT_OBJECT_QUAD_BITMAP,
        symbol: "Graph90_42_ConfigureCurrentObjectQuadBitmap",
        parameters: GRAPH90_CONFIGURE_CURRENT_OBJECT_QUAD_BITMAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47C1C0 -> sub_43DAB0: BackS seam position (1) then four bitmaps (2), 1.622 texts.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CONFIGURE_CURRENT_OBJECT_SPRITE_MASK,
        symbol: "Graph90_43_ConfigureCurrentObjectSpriteMask",
        parameters: GRAPH90_CONFIGURE_CURRENT_OBJECT_SPRITE_MASK_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47C2C0 -> sub_43DB10: BackF; transparency first, primary/secondary/mask codes 1..4, alpha only after success.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CONFIGURE_CURRENT_OBJECT_FRAME_TABLE,
        symbol: "Graph90_44_ConfigureCurrentObjectFrameTable",
        parameters: GRAPH90_CONFIGURE_CURRENT_OBJECT_FRAME_TABLE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47C430 -> sub_462700: BackD frame table 2..=32 (無効なビットマップ数) and the listed-bitmap error.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CONFIGURE_CURRENT_OBJECT_RESOURCE_TRIPLET,
        symbol: "Graph90_45_ConfigureCurrentObjectResourceTriplet",
        parameters: GRAPH90_CONFIGURE_CURRENT_OBJECT_RESOURCE_TRIPLET_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47C580 -> sub_462710: BackDst; transparency, then bitmap and vector maps in order with the 1.622 texts.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CONFIGURE_CURRENT_OBJECT_BITMAP_MODE,
        symbol: "Graph90_46_ConfigureCurrentObjectBitmapMode",
        parameters: GRAPH90_CONFIGURE_CURRENT_OBJECT_BITMAP_MODE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47C6E0 -> sub_462730: BackGrd; transparency, bitmap, size, gradation type.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CONFIGURE_CURRENT_OBJECT_VM_EFFECT,
        symbol: "Graph90_47_ConfigureCurrentObjectVmEffect",
        parameters: GRAPH90_CONFIGURE_CURRENT_OBJECT_VM_EFFECT_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47C7C0 -> sub_462740: BackRpl; transparency, bitmap, vector & distance map, ripple slot errors.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CONFIGURE_CURRENT_OBJECT_BITMAP_SIZE_POSITION,
        symbol: "Graph90_48_ConfigureCurrentObjectBitmapSizePosition",
        parameters: GRAPH90_CONFIGURE_CURRENT_OBJECT_BITMAP_SIZE_POSITION_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47C920 -> sub_462760: BackStr; bitmap, then reference range >= 2.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CONFIGURE_CURRENT_OBJECT_BITMAP_SIZE,
        symbol: "Graph90_49_ConfigureCurrentObjectBitmapSize",
        parameters: GRAPH90_CONFIGURE_CURRENT_OBJECT_BITMAP_SIZE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47C9F0 -> sub_462780: BackRtt; bitmap, then a zero scale is fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CONFIGURE_CURRENT_OBJECT_BLIT_SOURCES,
        symbol: "Graph90_4A_ConfigureCurrentObjectBlitSources",
        parameters: GRAPH90_CONFIGURE_CURRENT_OBJECT_BLIT_SOURCES_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47CA90 -> sub_462790: BackMsc; front/back bitmaps, style 0, linkage 0/1.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_CURRENT_OBJECT_RENDER_CONTROLS,
        symbol: "Graph90_4C_SetCurrentObjectRenderControls",
        parameters: GRAPH90_SET_CURRENT_OBJECT_RENDER_CONTROLS_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47CB90 -> sub_462890: draw/active pair kept by the manager and applied to the current background.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_GET_CURRENT_OBJECT_MODE,
        symbol: "Graph90_4D_GetCurrentObjectMode",
        parameters: GRAPH90_GET_CURRENT_OBJECT_MODE_PARAMETERS,
        returns: "i32 current-object mode",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47CBC0 -> sub_4628A0: the current background class.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CREATE_SPRITE_OBJECT,
        symbol: "Graph90_50_CreateSpriteObject",
        parameters: GRAPH90_CREATE_SPRITE_OBJECT_PARAMETERS,
        returns: "0x80000000-tagged sprite handle",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_47CBE0 -> sub_4628B0: first free of 512 sprite slots (0x80000000 | slot); none left is fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_RELEASE_SPRITE_OBJECT,
        symbol: "Graph90_51_ReleaseSpriteObject",
        parameters: GRAPH90_RELEASE_SPRITE_OBJECT_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47CC10: a cooperating procedure, an owner, or an unknown sprite are fatal (1.622 texts), else released.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_REFRESH_SPRITE_OBJECT,
        symbol: "Graph90_53_RefreshSpriteRect",
        parameters: GRAPH90_REFRESH_SPRITE_OBJECT_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47CC70 -> sub_462AB0: update rectangle; 10 invalid region, 255 unknown sprite.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_SPRITE_ENABLED,
        symbol: "Graph90_54_SetSpriteDrawEnabled",
        parameters: GRAPH90_SET_SPRITE_ENABLED_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47CCF0 -> sub_462AA0: draw enable (vtable+4); unknown sprite fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_SPRITE_AUX_BITMAP,
        symbol: "Graph90_55_SetSpriteAuxBitmap",
        parameters: GRAPH90_SET_SPRITE_AUX_BITMAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47CD30 -> sub_462A80: aux grey-scale bitmap; invalid (1), not screen-size grey-scale (2), unknown sprite (255).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CONFIGURE_SPRITE_SINGLE_BITMAP,
        symbol: "Graph90_56_ConfigureSpriteSingleBitmap",
        parameters: GRAPH90_CONFIGURE_SPRITE_SINGLE_BITMAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47CDD0 -> sub_4628D0: mode 0; priority, transparency, mode, bitmap checks; bitmap (1) and sprite (255) errors.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_REPLACE_SPRITE_BITMAP,
        symbol: "Graph90_57_ReplaceSpritePrimaryBitmap",
        parameters: GRAPH90_REPLACE_SPRITE_BITMAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47CEE0 -> sub_462A70: replaces the primary bitmap; bitmap (1), sprite (255).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CONFIGURE_SPRITE_DUAL_BITMAP,
        symbol: "Graph90_58_ConfigureSpriteDualBitmap",
        parameters: GRAPH90_CONFIGURE_SPRITE_DUAL_BITMAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47CF70 -> sub_4628F0: mode 1 pair; bitmaps, mixing ratio, transparency, priority; either invalid / size mismatch.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CONFIGURE_SPRITE_SCALED_BITMAP,
        symbol: "Graph90_59_ConfigureSpriteScaledBitmap",
        parameters: GRAPH90_CONFIGURE_SPRITE_SCALED_BITMAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47D0D0 -> sub_43EBB0/sub_427930: mode 2 scaled; checks, unknown sprite, bitmap, scaled extent below 2.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CONFIGURE_SPRITE_MASKED_BITMAP,
        symbol: "Graph90_5A_ConfigureSpriteMaskedBitmap",
        parameters: GRAPH90_CONFIGURE_SPRITE_MASKED_BITMAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47D270 -> sub_462960: mode 3 mask; checks, unknown sprite, either bitmap, unsuitable filter bitmap.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CONFIGURE_SPRITE_VM_EFFECT,
        symbol: "Graph90_5B_ConfigureSpriteVmEffect",
        parameters: GRAPH90_CONFIGURE_SPRITE_VM_EFFECT_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47D3F0 -> sub_462990: mode 4 VM effect; bitmaps, transparency, priority, sprite and map/ripple errors.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CONFIGURE_SPRITE_TRANSFORM_MODE5,
        symbol: "Graph90_5C_ConfigureSpriteTransformMode5",
        parameters: GRAPH90_CONFIGURE_SPRITE_TRANSFORM_MODE5_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47D710 -> sub_43EE70: mode 5; bitmap, mixing ratio, mode, transparency, priority; 9 (secondary mismatch) silent, 8 projection scale.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CONFIGURE_SPRITE_TRANSFORM_MODE6,
        symbol: "Graph90_5D_ConfigureSpriteTransformMode6",
        parameters: GRAPH90_CONFIGURE_SPRITE_TRANSFORM_MODE6_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47DA60 -> sub_462A10: mode 6; same checks; unknown sprite, bitmaps, projection scale.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CREATE_FILTER_OBJECT,
        symbol: "Graph90_60_CreateFilterObject",
        parameters: GRAPH90_CREATE_FILTER_OBJECT_PARAMETERS,
        returns: "0x90000000-tagged filter handle",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_47DDF0 -> sub_462AC0: first of 8 filter slots; none left is fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_RELEASE_FILTER_OBJECT,
        symbol: "Graph90_61_ReleaseFilterObject",
        parameters: GRAPH90_RELEASE_FILTER_OBJECT_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47DE20 -> sub_462AD0: release; unknown filter fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_FILTER_ENABLED,
        symbol: "Graph90_64_SetFilterEnabled",
        parameters: GRAPH90_SET_FILTER_ENABLED_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47DE50 -> sub_462B00: enable; unknown filter fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CONFIGURE_FILTER,
        symbol: "Graph90_65_ConfigureFilter",
        parameters: GRAPH90_CONFIGURE_FILTER_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47DE90 -> sub_462AE0(mask -1): priority and transparency checks, unknown filter fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CONFIGURE_FILTER_WITH_MASK,
        symbol: "Graph90_66_ConfigureFilterWithMask",
        parameters: GRAPH90_CONFIGURE_FILTER_WITH_MASK_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47DF00 -> sub_462AE0: masked filter; mask missing (1), not grey-scale (2), not screen size (3), unknown filter (255).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CREATE_MAP_OBJECT,
        symbol: "Graph90_70_CreateMapObject",
        parameters: GRAPH90_CREATE_MAP_OBJECT_PARAMETERS,
        returns: "0xA0000000-tagged map handle",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47E170 -> sub_462BC0: first of 8 map slots; none left is fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_RELEASE_MAP_OBJECT,
        symbol: "Graph90_71_ReleaseMapObject",
        parameters: GRAPH90_RELEASE_MAP_OBJECT_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47E1A0: release; unknown map fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_MAP_ENABLED,
        symbol: "Graph90_74_SetMapEnabled",
        parameters: GRAPH90_SET_MAP_ENABLED_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47E1D0: enable; unknown map fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CONFIGURE_MAP_OBJECT,
        symbol: "Graph90_75_ConfigureMapObject",
        parameters: GRAPH90_CONFIGURE_MAP_OBJECT_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47E210 -> sub_462C10: priority, transparency, mode, bitmap checks; unknown map, atlas unfit (1).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_INITIALIZE_MAP_GRID,
        symbol: "Graph90_76_InitializeMapGrid",
        parameters: GRAPH90_INITIALIZE_MAP_GRID_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47E320 -> sub_43FE30/sub_423E70: chip size 1..=256 (3) before view size within 1024x768 (2).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_UPLOAD_MAP_TILE_DATA,
        symbol: "Graph90_78_UploadMapTileData",
        parameters: GRAPH90_UPLOAD_MAP_TILE_DATA_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "47E400 ->424010: VM resolves BP pointer and copies width*height little-endian u16 entries; disables viewport and invalidates current cache IDs.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_MAP_VIEWPORT,
        symbol: "Graph90_79_SetMapViewport",
        parameters: GRAPH90_SET_MAP_VIEWPORT_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47E4A0 -> sub_43FF10: viewport (map, x, y, sub-x, sub-y, wrap); invalid coordinates (5), unknown map (255).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_REPLACE_MAP_TILE_ID,
        symbol: "Graph90_7A_ReplaceMapTileId",
        parameters: GRAPH90_REPLACE_MAP_TILE_ID_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47E570 -> sub_462C90: uninitialised map is fatal, unknown map fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CREATE_WINDOW_OBJECT,
        symbol: "Graph90_80_CreateWindowObject",
        parameters: GRAPH90_CREATE_WINDOW_OBJECT_PARAMETERS,
        returns: "0xB0000000-tagged window handle; target errors on invalid dimensions or exhausted 16-slot pool",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_47E5C0 -> sub_440970/sub_42B5D0: at most 16 windows; width < 32 / height < 20 count in 32-pixel units; 1..=1920 x 1..=32768 or fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_RELEASE_WINDOW_OBJECT,
        symbol: "Graph90_81_ReleaseWindowObject",
        parameters: GRAPH90_WINDOW_HANDLE_PARAMETER,
        returns: "void; target errors for invalid, procedure-bound, or owned windows",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47E660: cooperating procedure, owner, unknown window are fatal; else released.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_WINDOW_COMPOSITION_ORDER,
        symbol: "Graph90_82_SetWindowCompositionOrder",
        parameters: GRAPH90_SET_WINDOW_COMPOSITION_ORDER_PARAMETERS,
        returns: "void; target errors for modes outside 0..=5 or invalid window",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47E710 -> sub_463420: composition order 0..=5; unknown window (-1) and invalid order (1) fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_WINDOW_DRAW_ENABLED,
        symbol: "Graph90_84_SetWindowDrawEnabled",
        parameters: GRAPH90_SET_WINDOW_ENABLED_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47E790 -> sub_463200: draw enable; unknown window fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CONFIGURE_WINDOW_OBJECT,
        symbol: "Graph90_85_ConfigureWindowObject",
        parameters: GRAPH90_CONFIGURE_WINDOW_OBJECT_PARAMETERS,
        returns: "void; target validates blend, transparency, reserved value, priority and handle",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47E7D0 -> sub_463050: priority, two levels, blend mode checks; unknown window fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_WINDOW_ISOLATED_COMPOSITION,
        symbol: "Graph90_87_SetWindowIsolatedComposition",
        parameters: GRAPH90_SET_WINDOW_ISOLATED_COMPOSITION_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47E950 -> sub_463070: isolated composition; unknown window fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_WINDOW_VALID_REGION,
        symbol: "Graph90_88_SetWindowValidRegion",
        parameters: GRAPH90_SET_WINDOW_VALID_REGION_PARAMETERS,
        returns: "void; target errors when the inclusive rectangle exceeds the window",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47E990 -> sub_463080: valid region inside the window bitmap, else fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_GET_WINDOW_VALID_REGION,
        symbol: "Graph90_89_GetWindowValidRegion",
        parameters: GRAPH90_GET_WINDOW_VALID_REGION_PARAMETERS,
        returns: "one on success; target writes four i32 coordinates through out_rect",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47EA50 -> sub_4630C0: valid region into four DWORDs; unknown window fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_START_WINDOW_MESSAGE_PROCEDURE,
        symbol: "Graph90_90_StartWindowMessageProcedure",
        parameters: GRAPH90_START_WINDOW_MESSAGE_PROCEDURE_PARAMETERS,
        returns: "cooperative procedure result",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "sub_47EAA0: CProcDspMsg on a window; unknown window fatal; returns 2.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CONFIGURE_MESSAGE_CARET_FRAMES,
        symbol: "Graph90_98_ConfigureMessageCaretFrames",
        parameters: GRAPH90_CONFIGURE_CARET_FRAMES_PARAMETERS,
        returns: "void; target errors on an invalid bitmap entry",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47ECD0 -> sub_4336D0: caret frames; a bitmap that is missing or incompatible is fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_MESSAGE_CARET_FRAME_DELAY,
        symbol: "Graph90_99_SetMessageCaretFrameDelay",
        parameters: GRAPH90_CARET_DELAY_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47ED50 -> sub_433850: caret frame delay.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_MESSAGE_CARET_POSITION,
        symbol: "Graph90_9A_SetMessageCaretPosition",
        parameters: GRAPH90_CARET_POSITION_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47ED70 -> sub_433860: caret position triple.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_TEXT_SHADOW_ENABLED,
        symbol: "Graph90_9C_SetTextShadowEnabled",
        parameters: BOOLEAN_PARAMETER,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47EDE0 -> sub_4338F0: text shadow enable.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_TEXT_SHADOW_PARAMETERS,
        symbol: "Graph90_9D_SetTextShadowParameters",
        parameters: GRAPH90_SHADOW_PARAMETERS,
        returns: "void; target errors for offsets above 100 or concentration above 256",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47EE00 -> sub_463790: negative coordinates and a density above 256 are fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_REGISTER_FULLWIDTH_GLYPH_STRIP,
        symbol: "Graph90_9E_RegisterFullwidthGlyphStrip",
        parameters: GRAPH90_GLYPH_STRIP_PARAMETERS,
        returns: "void; target errors for count>255, invalid bitmap, or indivisible strip width",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47EEA0 -> sub_433320: full-width strip; count > 0xFF, missing bitmap, indivisible width fatal; 0 clears.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_START_ITEM_SELECTION,
        symbol: "Graph90_A0_StartItemSelection",
        parameters: GRAPH90_START_ITEM_SELECTION_PARAMETERS,
        returns: "cooperative selection result",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "BGI 1.622 sub_47EF80 (CProcSelectItem): window first, then item count 1..=16, initial item, columns 1..=16 with the 1.622 texts; returns 2.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_DRAW_ITEM_SELECTION_GRID,
        symbol: "Graph90_A1_DrawItemSelectionGrid",
        parameters: GRAPH90_DRAW_ITEM_SELECTION_GRID_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47F0F0 -> sub_492090: item grid draw; count, columns, window errors.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_START_ITEM_SELECTION_EX,
        symbol: "Graph90_A2_StartItemSelectionEx",
        parameters: GRAPH90_START_ITEM_SELECTION_EX_PARAMETERS,
        returns: "cooperative selection result",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "sub_47F410 (CProcSelectItemEx): as A0 plus the two highlight overlays.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_START_ITEM_SELECTION_EX_BLINK,
        symbol: "Graph90_A3_StartItemSelectionExBlink",
        parameters: GRAPH90_START_ITEM_SELECTION_EX_PARAMETERS,
        returns: "cooperative selection result",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "sub_47F430 (CProcSelectItemExBlink): as A2 with the confirmation blink.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_ITEM_SELECTION_HIGHLIGHT_STYLES,
        symbol: "Graph90_A4_SetItemSelectionHighlightStyles",
        parameters: GRAPH90_SELECTION_HIGHLIGHT_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47F450 -> sub_463900: highlight styles.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_ITEM_SELECTION_INPUT_MASK,
        symbol: "Graph90_A5_SetItemSelectionInputMask",
        parameters: GRAPH90_SELECTION_INPUT_MASK_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47F480 -> sub_463910: item input mask.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_ITEM_SELECTION_NAVIGATION_PARAMETERS,
        symbol: "Graph90_A6_SetItemSelectionNavigationParameters",
        parameters: GRAPH90_SELECTION_NAVIGATION_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47F4A0 -> sub_463920: navigation parameters.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_ITEM_SELECTION_COLUMN_LAYOUT,
        symbol: "Graph90_A7_SetItemSelectionColumnLayout",
        parameters: GRAPH90_SELECTION_COLUMN_LAYOUT_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47F4E0 -> sub_463940: column layout; unknown window fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_INTERACTIVE_PROCEDURE_POLL_GATE,
        symbol: "Graph90_AF_SetInteractiveProcedurePollGate",
        parameters: GRAPH90_INTERACTIVE_POLL_GATE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47F520 -> sub_463950: interactive procedure poll gate.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_START_ICON_SELECTION,
        symbol: "Graph90_B0_StartIconSelection",
        parameters: GRAPH90_START_ICON_SELECTION_PARAMETERS,
        returns: "cooperative selection result",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "sub_47F540 (CProcSelectIcon): window, icon count 1..=64, focus mode 0..=3 with the 1.622 texts.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_START_ICON_SELECTION_EX,
        symbol: "Graph90_B1_StartIconSelectionEx",
        parameters: GRAPH90_START_ICON_SELECTION_PARAMETERS,
        returns: "cooperative selection result",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "sub_47F650 (CProcSelectIconEx): 64-byte records, same checks.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_DRAW_ICON_BATCH,
        symbol: "Graph90_B4_DrawIconBatch",
        parameters: GRAPH90_DRAW_ICON_BATCH_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47F760 -> sub_4925D0: icon batch; count and window errors.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_DRAW_EXTENDED_ICON_BATCH,
        symbol: "Graph90_B5_DrawExtendedIconBatch",
        parameters: GRAPH90_DRAW_ICON_BATCH_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47F7F0: extended icon batch; count and window errors.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_APPLY_ICON_INPUT_LAYOUT,
        symbol: "Graph90_B6_ApplyIconInputLayout",
        parameters: GRAPH90_ICON_LAYOUT_PARAMETERS,
        returns: "status 0,1,2,3",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47F920 -> sub_46D270: input layout status.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_APPLY_ICON_INPUT_LAYOUT_EX,
        symbol: "Graph90_B7_ApplyIconInputLayoutEx",
        parameters: GRAPH90_ICON_LAYOUT_PARAMETERS,
        returns: "status 0,1,2,3",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47F960 -> sub_46D310: extended input layout status.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CREATE_ICON_INPUT_PROCESSOR,
        symbol: "Graph90_B8_CreateIconInputProcessor",
        parameters: GRAPH90_WINDOW_HANDLE_PARAMETER,
        returns: "DCIPIcon registry handle",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47F9A0 -> sub_46CB90: DCIPIcon, id ++dword_565D88 (from 1).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_RELEASE_ICON_INPUT_PROCESSOR,
        symbol: "Graph90_B9_ReleaseIconInputProcessor",
        parameters: GRAPH90_ICON_PROCESSOR_HANDLE_PARAMETER,
        returns: "one when released",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47F9D0 -> sub_46CC50: release, result pushed.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CONFIGURE_ICON_INPUT_PROCESSOR,
        symbol: "Graph90_BA_ConfigureIconInputProcessor",
        parameters: GRAPH90_CONFIGURE_ICON_PROCESSOR_PARAMETERS,
        returns: "status 0..=4",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47FA00 -> sub_46CE40: configuration status.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_GET_ICON_INPUT_STATE,
        symbol: "Graph90_BC_GetIconInputState",
        parameters: GRAPH90_ICON_STATE_OUTPUT_PARAMETERS,
        returns: "1 on valid handle, 0 on invalid handle",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47FA40: state record into the buffer, 1 when the processor exists.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_GET_ICON_INPUT_CURRENT_GROUP,
        symbol: "Graph90_BD_GetIconInputCurrentGroup",
        parameters: GRAPH90_ICON_CURRENT_GROUP_OUTPUT_PARAMETERS,
        returns: "1 on valid handle, 0 on invalid handle",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47FA80: current group, 1 when it exists.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_GET_ICON_INPUT_SELECTIONS,
        symbol: "Graph90_BE_GetIconInputSelections",
        parameters: GRAPH90_ICON_SELECTION_OUTPUT_PARAMETERS,
        returns: "1 on valid handle, 0 on invalid handle",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47FAC0: selections, 1 when it exists.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_POP_ICON_INPUT_EVENT,
        symbol: "Graph90_BF_PopIconInputEvent",
        parameters: GRAPH90_POP_ICON_INPUT_EVENT_PARAMETERS,
        returns: "1 for a valid processor handle, 0 for an invalid handle",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47FB00 -> sub_448560: pops one event (zeros when empty), 1 when it exists.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_LOAD_BG_BITMAP_RESOURCE,
        symbol: "Graph90_C0_LoadBgBitmapResource",
        parameters: GRAPH90_LOAD_BG_BITMAP_RESOURCE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_47FB40 -> sub_401EF0: only the BG/BMP header is read; the bitmap is created in its size and format (with the embedded point) and cleared to zero; missing file / non-BG fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_FLIP_BITMAP,
        symbol: "Graph90_C2_FlipBitmap",
        parameters: GRAPH90_FLIP_BITMAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47FC00 -> sub_405590: bitmap checks; source missing, destination equal to the source, the destination recreated before an invalid direction (0/1) is fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_DOWNSAMPLE_BITMAP_HALF,
        symbol: "Graph90_C3_DownsampleBitmapHalf",
        parameters: GRAPH90_BITMAP_PAIR_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47FCF0 -> sub_405090/sub_419220: half size, (a+b+c+d)>>2 truncating, edges >>1; source smaller than 2x2 fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_IMPORT_EXTERNAL_IMAGE,
        symbol: "Graph90_C4_ImportExternalImage",
        parameters: GRAPH90_IMPORT_EXTERNAL_IMAGE_PARAMETERS,
        returns: "status 0,1,2 or -1",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_46A300 uses GDI+ GdipCreateBitmapFromFile and writes a caller-owned descriptor; portable host decoding does not claim GDI+ layout equivalence.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SAVE_BITMAP_TO_IMAGE_FILE,
        symbol: "Graph90_C5_SaveBitmapToImageFile",
        parameters: GRAPH90_SAVE_BITMAP_PARAMETERS,
        returns: "status 0,4,5 or -1",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_46A600 maps format 0..4 to BMP/JPEG/GIF/TIFF/PNG and saves the selected current bitmap.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_REGISTER_BG_RESOURCE_DATA,
        symbol: "Graph90_C6_RegisterBgResourceData",
        parameters: GRAPH90_REGISTER_BG_RESOURCE_DATA_PARAMETERS,
        returns: "one on success",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47FF20 -> sub_401CB0: BG data registered under a lower-cased (namespace, name) key; 1 on success.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_LOAD_CACHED_BG_BITMAP,
        symbol: "Graph90_C7_LoadCachedBgBitmap",
        parameters: GRAPH90_LOAD_CACHED_BG_PARAMETERS,
        returns: "one on success",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47FF80 -> sub_401CE0: bitmap check, decode from the registered data in the BG's own pixel format; 1 on success.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_TRANSFORM_BITMAP_GENERAL,
        symbol: "Graph90_C8_TransformBitmapGeneral",
        parameters: GRAPH90_TRANSFORM_BITMAP_GENERAL_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47FFF0 -> sub_404FE0: perspective transform blit; destination and source errors with the 1.622 texts.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SCALE_BITMAP_ASPECT_FIT,
        symbol: "Graph90_CA_ScaleBitmapAspectFit",
        parameters: GRAPH90_BITMAP_PAIR_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4801C0 -> sub_405120: aspect-fit scale; destination then source must exist.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_REGISTER_TONE_CURVE,
        symbol: "Graph90_CC_RegisterToneCurve",
        parameters: GRAPH90_REGISTER_TONE_CURVE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_480260 -> sub_405420: tone curve registered from three control points (invalid points ignored); removing an unknown id is fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_APPLY_TONE_CURVE_EFFECT,
        symbol: "Graph90_CD_ApplyToneCurveEffect",
        parameters: GRAPH90_APPLY_TONE_CURVE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_405450 -> sub_4193D0 ported (bitmap_kernels::tone_curve) and matched under Unicorn with the record built by sub_40A060: monochrome tint (luminance 29/150/77 x colour, mixed by p3), the three-table tone-curve lookup, colour film (multiply < 128, screen above) mixed by p7; format 2 keeps the source alpha and clears fully transparent pairs (sub_4198E0), format 1 sub_419550. p6 (8 or 38) is only validated. The worker-thread band split is not reproduced.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_ENCODE_BITMAP_TO_BUFFER,
        symbol: "Graph90_CE_EncodeBitmapToBuffer",
        parameters: GRAPH90_ENCODE_BITMAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4056F0: checks and texts are the target's. Format 0 (sub_46A320, CompressedBG v1) and format 1 (sub_4A1930 -> sub_49C200, CompressedBG v2: float YCbCr, AAN float DCT with the quality tables of sub_49BE00, Huffman DC/AC per strip, LZ alpha, then the target's decode-and-compare checks) are ported (ethornell_image::encode_cbg_v1 / encode_cbg_v2) and byte-exact under Unicorn, including the encodings the checks reject (narrow 32-bit images whose alpha copies itself -> -2). Key = GetTickCount (the app's sound tick).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CREATE_KNOB_OBJECT,
        symbol: "Graph90_D0_CreateKnobObject",
        parameters: GRAPH90_CREATE_KNOB_PARAMETERS,
        returns: "0xF0000000 tagged knob handle",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_480620 -> sub_4424C0: a target with an owner (+0x130), 32 knobs, unknown target, virtual target class are fatal; the knob starts at the target's base position with its size as range.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_RELEASE_KNOB_OBJECT,
        symbol: "Graph90_D1_ReleaseKnobObject",
        parameters: GRAPH90_KNOB_HANDLE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4806A0: unwatches, releases; unknown knob fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_KNOB_ENABLED,
        symbol: "Graph90_D4_SetKnobEnabled",
        parameters: GRAPH90_KNOB_ENABLED_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4806E0 -> sub_4639C0: draw enable (vtable+4); unknown knob fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_KNOB_BASE_POSITION,
        symbol: "Graph90_D5_SetKnobBasePosition",
        parameters: GRAPH90_KNOB_PAIR_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_480720 -> sub_4639D0: base position; unknown knob fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_KNOB_POSITION,
        symbol: "Graph90_D6_SetKnobPosition",
        parameters: GRAPH90_KNOB_PAIR_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_480770 -> sub_4639E0: logical position; unknown knob fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_GET_KNOB_POSITION,
        symbol: "Graph90_D7_GetKnobPosition",
        parameters: GRAPH90_KNOB_HANDLE_PARAMETERS,
        returns: "two i32 values",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4807C0 -> sub_4639F0: pushes x and y; unknown knob fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_KNOB_MOVEMENT_PRECISION,
        symbol: "Graph90_D8_SetKnobMovementPrecision",
        parameters: GRAPH90_KNOB_PAIR_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_480810 -> sub_463A10: movement precision; unknown knob (-1), invalid precision (4) fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_KNOB_MOVEMENT_RANGE,
        symbol: "Graph90_D9_SetKnobMovementRange",
        parameters: GRAPH90_KNOB_PAIR_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4808A0 -> sub_463A50: movement range; unknown knob (-1), invalid range (5) fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_TAKE_KNOB_VERTICAL_EVENT,
        symbol: "Graph90_DA_TakeKnobVerticalEvent",
        parameters: GRAPH90_KNOB_HANDLE_PARAMETERS,
        returns: "i32 event delta",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_480930 -> sub_463A00: takes the vertical event; unknown knob fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_TAKE_CHANGED_KNOB_HANDLE,
        symbol: "Graph90_DB_TakeChangedKnobHandle",
        parameters: NO_PARAMETERS,
        returns: "knob handle or zero",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_480970 -> sub_463ED0: the changed knob handle.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_KNOB_RELATIVE_MODE,
        symbol: "Graph90_DC_SetKnobRelativeMode",
        parameters: GRAPH90_KNOB_MODE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_480990: relative mode; unknown knob fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SWAP_KNOB_INPUT_MODE,
        symbol: "Graph90_DD_SwapKnobInputMode",
        parameters: GRAPH90_SWAP_KNOB_INPUT_MODE_PARAMETERS,
        returns: "previous i32 mode",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4809D0 -> sub_463DA0: swaps the knob input mode, old value pushed.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_WATCH_KNOB_OBJECT,
        symbol: "Graph90_DE_WatchKnobObject",
        parameters: GRAPH90_KNOB_HANDLE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_480A00 -> sub_463E40: watch list (prepend); unknown knob fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_UNWATCH_KNOB_OBJECT,
        symbol: "Graph90_DF_UnwatchKnobObject",
        parameters: GRAPH90_KNOB_HANDLE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_480A30 -> sub_463DF0: unwatch; a knob not watched is fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CREATE_GROUP_OBJECT,
        symbol: "Graph90_E0_CreateGroupObject",
        parameters: NO_PARAMETERS,
        returns: "0xF1000000 tagged group handle",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_480A60 -> sub_463AA0: group slot; none left is fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_RELEASE_GROUP_OBJECT,
        symbol: "Graph90_E1_ReleaseGroupObject",
        parameters: GRAPH90_GROUP_HANDLE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_480A90 -> sub_463AB0: release, members detached; unknown group fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_GROUP_ENABLED,
        symbol: "Graph90_E4_SetGroupEnabled",
        parameters: GRAPH90_GROUP_ENABLED_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_480AC0 -> sub_463AC0: draw enable through members; unknown group fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CONFIGURE_GROUP_OBJECT,
        symbol: "Graph90_E5_ConfigureGroupObject",
        parameters: GRAPH90_CONFIGURE_GROUP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_480B00 -> sub_442AD0: transparency check, position and alpha through members; unknown group fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_ADD_OBJECT_TO_GROUP,
        symbol: "Graph90_E8_AddObjectToGroup",
        parameters: GRAPH90_ADD_GROUP_OBJECT_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_480B70 -> sub_463AF0: group, object, itself, existing owner are fatal; local offset kept.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_REMOVE_OBJECT_FROM_GROUP,
        symbol: "Graph90_E9_RemoveObjectFromGroup",
        parameters: GRAPH90_REMOVE_GROUP_OBJECT_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_480C00 -> sub_463B60: group, object, not a member are fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_OPEN_DIRECTSHOW_MOVIE,
        symbol: "Graph90_F0_OpenDirectShowMovie",
        parameters: GRAPH90_OPEN_DIRECTSHOW_MOVIE_PARAMETERS,
        returns: "duration in milliseconds, or zero when open fails",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_480C60: non-positive window size fatal; a missing movie file is the media-prompt error; opens the global DirectShow movie.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CLOSE_DIRECTSHOW_MOVIE,
        symbol: "Graph90_F1_CloseDirectShowMovie",
        parameters: NO_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_480D60: releases the global movie process.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_IS_DIRECTSHOW_MOVIE_PLAYING,
        symbol: "Graph90_F2_IsDirectShowMoviePlaying",
        parameters: NO_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_480D80: whether the global movie plays.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_SET_DIRECTSHOW_MOVIE_VOLUME,
        symbol: "Graph90_F3_SetDirectShowMovieVolume",
        parameters: GRAPH90_MOVIE_VOLUME_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_480DA0 -> sub_4901F0: volume <= 128 (unsigned) is stored and applied to the single global DirectShow movie unless the window is minimized (dword_566940); other values are ignored.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_LOAD_BURIKO_MOVIE_RESOURCE,
        symbol: "Graph90_F4_LoadBurikoMovieResource",
        parameters: GRAPH90_LOAD_BURIKO_MOVIE_PARAMETERS,
        returns: "no immediate value; procedure status 0/1/2",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "sub_480DC0 -> sub_452550: BURIKO movie load procedure (handle and metadata outputs); returns 2.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_RELEASE_BURIKO_MOVIE_RESOURCE,
        symbol: "Graph90_F5_ReleaseBurikoMovieResource",
        parameters: GRAPH90_BURIKO_MOVIE_HANDLE_PARAMETERS,
        returns: "0 on success or 3 for an invalid handle",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_480E70 -> sub_405CD0: release; 0 or 3.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_DECODE_BURIKO_MOVIE_FRAME,
        symbol: "Graph90_F6_DecodeBurikoMovieFrame",
        parameters: GRAPH90_DECODE_BURIKO_MOVIE_PARAMETERS,
        returns: "status 0/3/4/5/6; optionally through DCProcDecodeBMV",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "sub_480EC0 -> sub_405E00/sub_405F10: frame decode statuses 0/3/4/5/6, 2 while a thread decode runs.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_ATTACH_BURIKO_MOVIE_RESOURCE,
        symbol: "Graph90_F7_AttachBurikoMovieResource",
        parameters: GRAPH90_ATTACH_BURIKO_MOVIE_PARAMETERS,
        returns: "0, 3 for invalid source, or 7 when already attached",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_480FD0 -> sub_406120: attach; 0, 3 or 7.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_CLEAR_SPRITE_TARGETS,
        symbol: "Graph90_F8_ClearSpriteTargets",
        parameters: NO_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_481040 -> sub_496D90: clears every sprite target and the numbering.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_REGISTER_SPRITE_TARGET,
        symbol: "Graph90_FA_RegisterSpriteTarget",
        parameters: GRAPH90_SPRITE_TARGET_PARAMETERS,
        returns: "void; target raises a script error for a non-Sprite handle",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_481050 -> sub_496DC0: registers a sprite target; a non-sprite handle is fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_UNREGISTER_SPRITE_TARGET,
        symbol: "Graph90_FB_UnregisterSpriteTarget",
        parameters: GRAPH90_SPRITE_TARGET_PARAMETERS,
        returns: "void; target raises a script error when not registered",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_481080 -> sub_496E20: unregisters; an unregistered handle is fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_HIT_TEST_SPRITE_TARGETS,
        symbol: "Graph90_FC_HitTestSpriteTargets",
        parameters: NO_PARAMETERS,
        returns: "target number or -1",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4810B0 -> sub_496E70: target number under the pointer or -1.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH90_GET_SPRITE_TARGET_STATE,
        symbol: "Graph90_FD_GetSpriteTargetState",
        parameters: GRAPH90_SPRITE_TARGET_NUMBER_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4810D0 -> sub_496F20: sampled state; an invalid target number is fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH_RENDER_OBJECT_TO_BITMAP,
        symbol: "Graph90_83_RenderWindowToBitmap",
        parameters: GRAPH_RENDER_OBJECT_TO_BITMAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47E6C0 -> sub_4633E0: bitmap check, window rendered into the bitmap; unknown window fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH_BIND_BITMAP_TO_SURFACE,
        symbol: "Graph90_86_SetWindowBackgroundBitmap",
        parameters: GRAPH_BIND_BITMAP_TO_SURFACE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47E880 -> sub_462EF0: frame bitmaps; unknown window, missing resources, unregistered bitmap errors.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_CONFIGURE_COMPACT_WAVE_TABLE,
        symbol: "Graph92_00_ConfigureCompactWaveTable",
        parameters: GRAPH92_CONFIGURE_COMPACT_WAVE_TABLE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_485FA0 -> sub_409AE0: slot < 8 (fatal text), quarter period, unsigned amplitude, zero rows after each cycle; cvttsd2si then low 16 bits.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_CONFIGURE_WAVE_TABLE,
        symbol: "Graph92_01_ConfigureWaveTable",
        parameters: GRAPH92_CONFIGURE_WAVE_TABLE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_486050 -> sub_409C50: slot < 8, zero rows first, lead levels 1<<n, main cycle, trail levels 2^n.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_GENERATE_RADIAL_VECTOR_MAP,
        symbol: "Graph92_10_GenerateRadialVectorMap",
        parameters: GRAPH92_RADIAL_VECTOR_MAP_PARAMETERS,
        returns: "void; target raises status 1/3/9 for invalid descriptor, format, or mode",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_486120 -> sub_404CA0: missing, not a format-6 map, mode < 2 with target texts; 32767*d/dist vectors, phase (4*dist) mod 4*period as unsigned (0 -> 0xFFFFFFFF).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_GENERATE_AXIS_VECTOR_MAP,
        symbol: "Graph92_11_GenerateAxisVectorMap",
        parameters: GRAPH92_AXIS_VECTOR_MAP_PARAMETERS,
        returns: "void; target raises status 1/3/10 for invalid descriptor, format, or mode",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_486220 -> sub_404E70: same checks, mode < 4.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_SET_BITMAP_AUXILIARY_PAIR,
        symbol: "Graph92_12_SetBitmapAuxiliaryPair",
        parameters: GRAPH92_BITMAP_AUXILIARY_PAIR_PARAMETERS,
        returns: "i32 boolean",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4862F0 -> sub_402440: descriptor +0x28/+0x2C = p1/p2; returns found.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_REPLACE_BITMAP_COLOR,
        symbol: "Graph92_13_ReplaceBitmapColor",
        parameters: GRAPH92_REPLACE_BITMAP_COLOR_PARAMETERS,
        returns: "status 0/1/2",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_486330 -> sub_4024A0: missing 1, not 4 bytes 2; format 1/2 colour replacement rules.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_PRELOAD_BITMAP_RESOURCE,
        symbol: "Graph92_14_PreloadBitmapResource",
        parameters: GRAPH92_PRELOAD_BITMAP_PARAMETERS,
        returns: "through DCProcPreloadBmp",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "0x00485870 allocates DCProcPreloadBmp and returns scheduler status 2. Only the first converted string reaches the preload core in this build.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_CANCEL_PENDING_BITMAP_PRELOADS,
        symbol: "Graph92_15_CancelPendingBitmapPreloads",
        parameters: NO_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "0x00485900 -> sub_4504F0 removes every pending DCProcPreloadBmp node.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_GET_BITMAP_AUXILIARY_PAIR,
        symbol: "Graph92_16_GetBitmapAuxiliaryPair",
        parameters: GRAPH92_GET_BITMAP_AUXILIARY_PAIR_PARAMETERS,
        returns: "i32 boolean",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_486410 -> sub_402470: writes +0x28/+0x2C.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_READ_BITMAP_PIXEL_VALUE,
        symbol: "Graph92_17_ReadBitmapPixelValue",
        parameters: GRAPH92_READ_BITMAP_PIXEL_VALUE_PARAMETERS,
        returns: "status 0/1/2/3",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_486450 -> sub_4025E0: missing 1, >4 bytes 2, coordinates 3; output cleared only on success.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH_CONVERT_BITMAP_TO_ALPHA_DESCRIPTOR,
        symbol: "Graph92_18_ConvertBitmapToAlphaDescriptor",
        parameters: GRAPH_CONVERT_BITMAP_TO_ALPHA_DESCRIPTOR_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4864A0 -> sub_408310: missing source 9, destination outside 10; other formats leave a blank format-3 destination.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_INVERT_ALPHA_BITMAP,
        symbol: "Graph92_19_InvertAlphaBitmap",
        parameters: GRAPH92_INVERT_ALPHA_BITMAP_PARAMETERS,
        returns: "i32 boolean",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_486530 -> sub_408390: format-3 coverage inverted.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_COMPOSE_BITMAP_ALPHA_AT_OFFSET,
        symbol: "Graph92_1A_ComposeBitmapAlphaAtOffset",
        parameters: GRAPH92_COMPOSE_ALPHA_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_486560 -> sub_403C20: source, second source, screen-sized destination, pixel-mode match with target texts.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_DRAW_BITMAP_TEXT_MEASURE,
        symbol: "Graph92_1C_DrawBitmapTextAndMeasureAdvance",
        parameters: GRAPH92_BITMAP_TEXT_MEASURE_PARAMETERS,
        returns: "largest line advance",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4866A0 -> sub_403600: bitmap validator, font number, bitmap exists, size, width; glyphs are a renderer item.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_DRAW_WRAPPED_BITMAP_TEXT,
        symbol: "Graph92_1D_DrawWrappedBitmapTextAndCountLines",
        parameters: GRAPH92_BITMAP_TEXT_WRAP_PARAMETERS,
        returns: "wrapped line count",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_486850 -> sub_403680: same checks as 1C.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_DRAW_BITMAP_TEXT,
        symbol: "Graph92_1E_DrawBitmapText",
        parameters: GRAPH92_DRAW_BITMAP_TEXT_PARAMETERS,
        returns: "text advance",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_486A10 -> sub_4039E0: bitmap validator, font, bitmap, height 8..=200.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_LOAD_EXTERNAL_BMP,
        symbol: "Graph92_1F_LoadExternalBmp",
        parameters: GRAPH92_LOAD_EXTERNAL_BMP_PARAMETERS,
        returns: "0 on success or negative parser/load status",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "0x00486090 searches the resource/filesystem path and parses an uncompressed BMP into the supplied destination descriptor; it does not consume raw BP image bytes.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH_SET_MESSAGE_INPUT_SCOPE,
        symbol: "Graph90_91_SetMessageInputScope",
        parameters: MESSAGE_INPUT_SCOPE_PARAMETERS,
        returns: "void; target emits a script error for invalid mode/value",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47EB40 -> sub_433150: input scope mode 0..=2, mode 1 priority < 0x10000, both fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH_SET_MESSAGE_INPUT_FILTER,
        symbol: "Graph90_92_SetMessageInputFilter",
        parameters: MESSAGE_INPUT_FILTER_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47EBE0 -> sub_433190: dword_565BBC.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_CACHE_BINARY_RESOURCE,
        symbol: "Graph91_03_CacheBinaryResource",
        parameters: GRAPH91_CACHE_BINARY_RESOURCE_PARAMETERS,
        returns: "0 on success or -1 when the native cache is disabled or insertion fails",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_481180 -> sub_401ED0/sub_446310: stores the bytes in the DCCache (only while Graph90:03 set one; size up to half the budget, LRU eviction); 0 or -1.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SET_GLOBAL_DISPLAY_OFFSET,
        symbol: "Graph91_06_SetGlobalDisplayOffset",
        parameters: GRAPH91_DISPLAY_OFFSET_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4811D0 -> sub_41A9C0: global display offset dword_565B44/48, screen marked dirty.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SET_SCRIPT_BITMAP_CONTEXT_BINDING_ENABLED,
        symbol: "Graph91_0B_SetScriptBitmapContextBindingEnabled",
        parameters: GRAPH91_BOOLEAN_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_481200 -> sub_48DD00: dword_503EFC script bitmap-context binding.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SET_GLYPH_COVERAGE_MODE,
        symbol: "Graph91_0C_SetGlyphCoverageMode",
        parameters: GRAPH91_GLYPH_COVERAGE_MODE_PARAMETERS,
        returns: "1 for accepted mode 0 or 1, otherwise 0",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_481220 -> sub_42E010: glyph coverage mode 0/1 (unsigned), 1 when accepted.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SET_FONT_PITCH_DETECTION_ENABLED,
        symbol: "Graph91_0D_SetFontPitchDetectionEnabled",
        parameters: GRAPH91_BOOLEAN_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_481250 -> sub_42E000: dword_565B70 pitch detection.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_REGISTER_NAMED_FONT_TRANSFORM,
        symbol: "Graph91_0E_RegisterNamedFontTransform",
        parameters: GRAPH91_FONT_TRANSFORM_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_481270 -> sub_42F1B0/sub_42EF10: named font transform; scale and offset limits fatal with the 1.622 texts.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_CONFIGURE_NATIVE_FONT,
        symbol: "Graph91_0F_ConfigureNativeFont",
        parameters: GRAPH91_NATIVE_FONT_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_481360 -> sub_462310: unknown face, size, width are fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_GENERATE_AFFINE_DISPLACEMENT_MAP,
        symbol: "Graph91_10_GenerateAffineDisplacementMap",
        parameters: GRAPH91_AFFINE_MAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_481470 -> sub_403D10: zoom vector map; destination missing / not a vector map / zero range fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_GENERATE_RANDOM_DISPLACEMENT_MAP,
        symbol: "Graph91_11_GenerateRandomDisplacementMap",
        parameters: GRAPH91_RANDOM_MAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_481580 -> sub_403F00: random vector map; destination checks.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_GENERATE_RIPPLE_DISPLACEMENT_MAP,
        symbol: "Graph91_12_GenerateRippleDisplacementMap",
        parameters: GRAPH91_RIPPLE_MAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_481620 -> sub_404130: ripple vector map; destination checks, zero period fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_GENERATE_PERSPECTIVE_BEND_MAP,
        symbol: "Graph91_13_GeneratePerspectiveBendMap",
        parameters: GRAPH91_FIVE_MAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_481730 -> sub_4044E0: perspective bend map; destination checks.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_GENERATE_CURVATURE_DISPLACEMENT_MAP,
        symbol: "Graph91_14_GenerateCurvatureDisplacementMap",
        parameters: GRAPH91_FIVE_MAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_481800 -> sub_404530: curvature map; destination checks, zero angle or radius fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_GENERATE_RADIAL_LENS_DISPLACEMENT_MAP,
        symbol: "Graph91_15_GenerateRadialLensDisplacementMap",
        parameters: GRAPH91_FIVE_MAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_481910 -> sub_404740: radial lens map; destination checks, zero strength or radius fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_GENERATE_SINE_DISPLACEMENT_MAP,
        symbol: "Graph91_16_GenerateSineDisplacementMap",
        parameters: GRAPH91_SINE_MAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_481A20 -> sub_404AB0: sine map; destination checks.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_GENERATE_RADIAL_WARP_DISPLACEMENT_MAP,
        symbol: "Graph91_17_GenerateRadialWarpDisplacementMap",
        parameters: GRAPH91_RADIAL_WARP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_481B20 -> sub_404C50: radial warp map; destination checks.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_COMPOSITE_BITMAP_RECT_ALPHA,
        symbol: "Graph91_18_CompositeBitmapRectAlpha",
        parameters: GRAPH91_RECT_COMPOSITE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_402F30 -> sub_416B60 ported (bitmap_kernels::rotate_blit) and matched under Unicorn: the plain-blit path (whole anchors, unit scales, 360k degrees, source covering the destination -> sub_40B560), else a format-1 destination only: sub_4169D0's x87 walk (53-bit precision) and sub_418500/sub_4173B0 (nearest), sub_417ED0/sub_416E20 (4-bit bilinear), sub_4177E0/sub_4174E0 (format-2 source). The target's worker-thread band split (sub_41A120) is not reproduced.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_COMPOSITE_BITMAP_RECT_CONVERTED,
        symbol: "Graph91_19_CompositeBitmapRectConverted",
        parameters: GRAPH91_RECT_COMPOSITE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_402FD0 -> sub_4179B0 ported (bitmap_kernels::rotate_copy) and matched under Unicorn: a copy faded by 256 - alpha over the whole destination. Plain path sub_418A80 (sub_40AF70 / sub_418AB0); equal formats 1/2 through sub_418500/sub_4183E0 (nearest) and sub_417ED0/sub_417C20 (bilinear), alpha 256 clears (sub_40A610); format 1 into format 2 opaque (sub_4185F0 / sub_418140). The worker-thread band split (sub_41A120) is not reproduced.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_REPLACE_BITMAP_RGB_PRESERVE_ALPHA,
        symbol: "Graph91_1A_ReplaceBitmapRgbPreserveAlpha",
        parameters: GRAPH91_REPLACE_RGB_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_418B50: destination = colour | source alpha over the overlap, both format 2 (checked against the decompile).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_CONCENTRATE_BITMAP,
        symbol: "Graph91_1B_ConcentrateBitmap",
        parameters: GRAPH91_CONCENTRATE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_418F60 ported literally (bitmap_kernels::concentrate) and matched under Unicorn (src/tools/emu_kernels.py); both bitmaps format 1, x factor p3, y factor p2.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SCALE_TRUE_COLOR_BITMAP,
        symbol: "Graph91_1C_ScaleTrueColorBitmap",
        parameters: GRAPH91_SCALE_TRUE_COLOR_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_402B90 reallocates the destination at (w*p2>>16, h*p3>>16) in the source format; sub_495880 (p4 != 0: bilinear / box / area sampler by scale) and sub_495680 (nearest) ported literally (bitmap_kernels::scale_filtered / scale_nearest) and matched under Unicorn (src/tools/emu_kernels.py).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_PROCESS_BITMAP,
        symbol: "Graph91_1D_ProcessBitmap",
        parameters: GRAPH91_PROCESS_BITMAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_403110 -> sub_419CF0 ported literally for 32-bit bitmaps (bitmap_kernels::process: 0 copy/convert sub_40AF70, 1 XOR sub_410D80/sub_410F80, 2 mono tint sub_4111E0/sub_4113B0, 3 fade sub_4116F0 or colour 0 sub_40DE80, 4/5 saturating add/sub sub_4118E0/sub_411A00) and matched under Unicorn (src/tools/emu_kernels.py). The RGB555 branches (format 0) have no app raster.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_APPLY_GRAYSCALE_MASK,
        symbol: "Graph91_1E_ApplyGrayscaleMask",
        parameters: GRAPH91_GRAYSCALE_MASK_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_403190 clips target against the source moved by (-p2, -p3) and runs sub_415E30 with the target as input and output: format-2 source -> sub_4164F0 (alpha scaled by the source alpha, 255 kept), format-3 source -> sub_415EC0 (format-3 target) or sub_416020 (format-2 target alpha); other formats are ignored error codes. Ported literally (bitmap_kernels::grayscale_mask) and matched under Unicorn through sub_403190 itself (src/tools/emu_kernels.py).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_CLONE_BITMAP,
        symbol: "Graph91_1F_CloneBitmap",
        parameters: GRAPH91_CLONE_BITMAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "BGI 1.622 sub_4823D0 -> sub_403450: clone (format, size, embedded point); missing source (2) or a destination that cannot be created (1) fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SET_OBJECT_SUPPRESSED,
        symbol: "Graph91_31_SetObjectSuppressed",
        parameters: GRAPH91_OBJECT_BOOLEAN_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_482460 -> sub_462490: draw suppression; unknown object fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SET_OBJECT_FIXED_POSITION,
        symbol: "Graph91_33_SetObjectFixedPosition",
        parameters: GRAPH91_OBJECT_FIXED_POSITION_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4824A0 -> sub_4624C0: fixed position; unknown object fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SET_OBJECT_SECONDARY_VECTOR,
        symbol: "Graph91_36_SetObjectSecondaryVector",
        parameters: GRAPH91_OBJECT_VECTOR_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4824F0 -> sub_462550: secondary vector; unknown object fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SET_OBJECT_PRIMARY_VECTOR,
        symbol: "Graph91_37_SetObjectPrimaryVector",
        parameters: GRAPH91_OBJECT_VECTOR_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_482540 -> sub_462530: primary vector; unknown object fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_GET_OBJECT_PROPERTY,
        symbol: "Graph91_38_GetObjectProperty",
        parameters: GRAPH91_GET_OBJECT_PROPERTY_PARAMETERS,
        returns: "void; target reports invalid object/output or unsupported parameter",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_482590 -> sub_462590: property getter into the DWORD; 5 unsupported, 254 bad argument, 255 unknown object with the 1.622 texts.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_GET_OBJECT_COMPOSITE_POSITION,
        symbol: "Graph91_3D_GetObjectCompositePosition",
        parameters: GRAPH91_GET_OBJECT_POSITION_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_482650 -> sub_462650: composite position into two DWORDs; unknown object fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_ATTACH_CHILD_OBJECT,
        symbol: "Graph91_3E_AttachChildObject",
        parameters: GRAPH91_ATTACH_OBJECT_PARAMETERS,
        returns: "void; target emits script errors for invalid/self/already-owned objects",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_482690 -> sub_462660: attach slave; master (255), slave (6), itself (7), slave owner (8) fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_DETACH_CHILD_OBJECT,
        symbol: "Graph91_3F_DetachChildObject",
        parameters: GRAPH91_DETACH_OBJECT_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_482830 -> sub_462680: detach; master, slave, not subordinate (9) fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_INITIALIZE_MULTILAYER_BACKGROUND,
        symbol: "Graph91_40_InitializeMultiLayerBackground",
        parameters: GRAPH91_MULTILAYER_INITIALIZE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_482890 -> sub_4627B0: multi-layer background; bitmap check, invalid bitmap (3), zero scale (4).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SELECT_MULTILAYER_LAYER,
        symbol: "Graph91_41_SelectMultiLayer",
        parameters: GRAPH91_MULTILAYER_INDEX_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4829C0 -> sub_4627E0: selects a layer; not multi-layer mode (1), layer (2) fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SET_MULTILAYER_LAYER_ENABLED,
        symbol: "Graph91_42_SetMultiLayerEnabled",
        parameters: GRAPH91_MULTILAYER_ENABLED_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_482A30 -> sub_4627F0: layer enable; mode and layer checks.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SET_MULTILAYER_LAYER_POSITION,
        symbol: "Graph91_43_SetMultiLayerPosition",
        parameters: GRAPH91_MULTILAYER_PAIR_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_482AB0 -> sub_462800: layer position; mode and layer checks.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SET_MULTILAYER_LAYER_BLEND_MODE,
        symbol: "Graph91_44_SetMultiLayerBlendMode",
        parameters: GRAPH91_MULTILAYER_SCALAR_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_482B40 -> sub_462810: blend mode check, mode and layer checks.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SET_MULTILAYER_LAYER_BLEND_PARAMETER,
        symbol: "Graph91_45_SetMultiLayerBlendParameter",
        parameters: GRAPH91_MULTILAYER_SCALAR_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_482BD0 -> sub_462820: level check, mode and layer checks.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SET_MULTILAYER_LAYER_BITMAP,
        symbol: "Graph91_46_SetMultiLayerBitmap",
        parameters: GRAPH91_MULTILAYER_BITMAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_482C60 -> sub_462830: bitmap check, mode, layer, bitmap errors.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SET_MULTILAYER_LAYER_TRANSFORM,
        symbol: "Graph91_47_SetMultiLayerTransform",
        parameters: GRAPH91_MULTILAYER_TRANSFORM_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_482D30 -> sub_462840: mode, layer, zero scale errors.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SET_MULTILAYER_LAYER_AUXILIARY_PAIR,
        symbol: "Graph91_48_SetMultiLayerAuxiliaryPair",
        parameters: GRAPH91_MULTILAYER_PAIR_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_482E10 -> sub_462860: auxiliary pair; mode and layer checks.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SET_MULTILAYER_LAYER_SOURCE_VELOCITY,
        symbol: "Graph91_49_SetMultiLayerSourceVelocity",
        parameters: GRAPH91_MULTILAYER_PAIR_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_482EA0 -> sub_462870: source velocity; mode and layer checks.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SET_MULTILAYER_LAYER_TRANSFORM_VELOCITY,
        symbol: "Graph91_4A_SetMultiLayerTransformVelocity",
        parameters: GRAPH91_MULTILAYER_TRANSFORM_VELOCITY_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_482F30 -> sub_462880: transform velocity; mode and layer checks.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SET_SPRITE_RELATION,
        symbol: "Graph91_55_SetSpriteRelation",
        parameters: GRAPH91_SPRITE_RELATION_PARAMETERS,
        returns: "i32 status",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_482FD0 -> sub_43F160 -> sub_429070: sprite=p0, linked=p1; 255 missing sprite, 11 self/missing link, 13 already linked or unlink without +0x154/+0x160, 14 linked sprite already claimed; linking first runs sub_428320(-1).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_CREATE_EFFECTOR,
        symbol: "Graph91_60_CreateEffector",
        parameters: NO_PARAMETERS,
        returns: "0x91000000 tagged handle or 0",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_483010 -> sub_43FBB0: 8 slots, exhaustion is a script error.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_RELEASE_EFFECTOR,
        symbol: "Graph91_61_ReleaseEffector",
        parameters: GRAPH91_EFFECTOR_HANDLE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_483040 -> sub_43F750: unknown effector is a script error.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SET_EFFECTOR_ENABLED,
        symbol: "Graph91_64_SetEffectorEnabled",
        parameters: GRAPH91_EFFECTOR_ENABLED_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_483070 -> sub_43FAF0: enable flag; unknown effector is a script error.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_CONFIGURE_DUAL_VECTOR_EFFECTOR,
        symbol: "Graph91_65_ConfigureDualVectorMapEffector",
        parameters: GRAPH91_DUAL_VECTOR_EFFECTOR_PARAMETERS,
        returns: "void; target reports map validation errors",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4830B0 effector configure: priority/alpha validators first, then bitmap/vector map errors with full multi-line texts.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_CONFIGURE_GRADIENT_EFFECTOR,
        symbol: "Graph91_66_ConfigureGradientEffector",
        parameters: GRAPH91_GRADIENT_EFFECTOR_PARAMETERS,
        returns: "void; target rejects gradient types outside 0..=1",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "effector configure (4 args): validators first, target texts.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_CONFIGURE_RIPPLE_EFFECTOR,
        symbol: "Graph91_67_ConfigureRippleEffector",
        parameters: GRAPH91_RIPPLE_EFFECTOR_PARAMETERS,
        returns: "void; target validates map, maximum distance and ripple registration",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "effector V&D map configure: validators, full multi-line texts.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_CONFIGURE_TRANSFORM_EFFECTOR,
        symbol: "Graph91_68_ConfigureTransformEffector",
        parameters: GRAPH91_TRANSFORM_EFFECTOR_PARAMETERS,
        returns: "void; target rejects zero scales",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "effector configure (9 args): validators, target texts.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_CONFIGURE_SURFACE_EFFECTOR,
        symbol: "Graph91_69_ConfigureSurfaceEffector",
        parameters: GRAPH91_SURFACE_EFFECTOR_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "effector configure (3 args): validators, target texts.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_CREATE_LANDSCAPE,
        symbol: "Graph91_70_CreateLandscape",
        parameters: GRAPH91_CREATE_LANDSCAPE_PARAMETERS,
        returns: "0xA1000000 tagged handle or 0",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4837F0 -> sub_440010 -> sub_421AB0: six arguments; four slots, exhaustion is a script error.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_RELEASE_LANDSCAPE,
        symbol: "Graph91_71_ReleaseLandscape",
        parameters: GRAPH91_LANDSCAPE_HANDLE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_483870 -> sub_440130: unknown landscape is a script error.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_HIT_TEST_LANDSCAPE,
        symbol: "Graph91_73_HitTestLandscapeAtPointer",
        parameters: GRAPH91_LANDSCAPE_HIT_TEST_PARAMETERS,
        returns: "i32 boolean",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4838A0 -> sub_462CD0 -> sub_4401B0 -> sub_422C50: 255/13 fatal; hit writes [column, line]; 8-bit parts test their byte.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SET_LANDSCAPE_ENABLED,
        symbol: "Graph91_74_SetLandscapeEnabled",
        parameters: GRAPH91_LANDSCAPE_ENABLED_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_483910 -> sub_440220: vtable+4 draw-enable (sub_41B120); unknown landscape fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_CONFIGURE_LANDSCAPE_OBJECT,
        symbol: "Graph91_75_ConfigureLandscapeObject",
        parameters: GRAPH91_LANDSCAPE_OBJECT_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_483950: priority, alpha, blend mode validators; sub_423380 refuses a priority whose last row passes 0xFFFF and then nothing else changes.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SET_LANDSCAPE_CELL_SILHOUETTE,
        symbol: "Graph91_76_SetLandscapeCellSilhouette",
        parameters: GRAPH91_LANDSCAPE_SILHOUETTE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4839F0 -> sub_422A60: map 13, line 11, column 12, type 18, level 19 with target texts.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_CONFIGURE_LANDSCAPE_PARTS,
        symbol: "Graph91_78_ConfigureLandscapePartsAndColumns",
        parameters: GRAPH91_LANDSCAPE_PARTS_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_483C20 -> sub_4403C0 -> sub_421C60: part/pillar count -1 details, bitmap, unsigned part rect containment, pillar record checks; error index kept at CDspObj+0x118.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_CONFIGURE_LANDSCAPE_MAP,
        symbol: "Graph91_79_ConfigureLandscapeMap",
        parameters: GRAPH91_LANDSCAPE_MAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_483E90 -> sub_422190: pillars 4, size and last-row priority 5, entries 6.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_CONFIGURE_LANDSCAPE_GUIDES,
        symbol: "Graph91_7A_ConfigureLandscapeGuides",
        parameters: GRAPH91_LANDSCAPE_GUIDES_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_484060 -> sub_422800: count, bitmap, guide rect checks.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SET_LANDSCAPE_CELL_GUIDES,
        symbol: "Graph91_7B_SetLandscapeCellGuides",
        parameters: GRAPH91_LANDSCAPE_CELL_GUIDES_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_484140: value validator, then per-pair sub_4229B0 checks (map, line, column, layer, guide); earlier pairs stay applied.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_COPY_LANDSCAPE_PART,
        symbol: "Graph91_7C_CopyLandscapePart",
        parameters: GRAPH91_LANDSCAPE_PART_COPY_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_484390 -> sub_422380: pillars 4, destination 8, source/incompatible 9.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SET_LANDSCAPE_CELL_COLUMN,
        symbol: "Graph91_7D_SetLandscapeMapCellColumn",
        parameters: GRAPH91_LANDSCAPE_CELL_COLUMN_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_484590 -> sub_4225D0: cell checks then pillar 10; sub_4237D0 clears guides only on visible pillar cells.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_GET_LANDSCAPE_CELL_VALUE,
        symbol: "Graph91_7E_GetLandscapeCellValue",
        parameters: GRAPH91_LANDSCAPE_CELL_VALUE_PARAMETERS,
        returns: "i32 boolean",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4847A0 -> sub_422BE0: cell checks fatal, empty cell returns false.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_COPY_LANDSCAPE_CELL_IMAGE,
        symbol: "Graph91_7F_CopyLandscapeCellImage",
        parameters: GRAPH91_LANDSCAPE_CELL_IMAGE_PARAMETERS,
        returns: "i32 boolean",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4849A0 -> sub_422AF0: cell checks, empty cell false, destination allocation 15.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_CONFIGURE_WINDOW_FONT,
        symbol: "Graph91_88_ConfigureWindowFont",
        parameters: GRAPH91_WINDOW_FONT_PARAMETERS,
        returns: "void; target reports native font/window validation errors",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_484BA0: sub_498660 font number, window, sub_42EE90 size 4..=200 and width 25..=200 with target texts; render option narrows the valid rect.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SET_WINDOW_LINE_SPACING,
        symbol: "Graph91_89_SetWindowLineSpacingPercent",
        parameters: GRAPH91_WINDOW_LINE_SPACING_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_484CD0 -> sub_42C870: window, spacing <= 800 fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SET_WINDOW_MESSAGE_VARIANT,
        symbol: "Graph91_8A_SetWindowMessageVariant",
        parameters: MESSAGE_VARIANT_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_484D50 -> sub_42C930: window, variant <= 1 fatal, then sub_42C990 cursor reset (left/top or right/top).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SET_WINDOW_TEXT_LAYOUT_MODE,
        symbol: "Graph91_8B_SetWindowTextLayoutMode",
        parameters: GRAPH91_WINDOW_LAYOUT_MODE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_484DD0 -> sub_42C960: window, mode <= 2 fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SET_WINDOW_TEXT_CURSOR,
        symbol: "Graph91_8C_SetWindowTextCursor",
        parameters: GRAPH91_WINDOW_CURSOR_SET_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_484E50 -> sub_42CAE0: window fatal; cursor +0x384/+0x388.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_GET_WINDOW_TEXT_CURSOR,
        symbol: "Graph91_8D_GetWindowTextCursor",
        parameters: GRAPH91_WINDOW_HANDLE_PARAMETERS,
        returns: "three outputs: valid, cursor_x, cursor_y",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_484EA0: window fatal; pushes 1, x, y.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_IS_WINDOW_TEXT_CURSOR_AT_BOUNDARY,
        symbol: "Graph91_8E_IsWindowTextCursorAtBoundary",
        parameters: GRAPH91_WINDOW_HANDLE_PARAMETERS,
        returns: "i32 boolean",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_484EF0 -> sub_42CB20: window fatal; boundary is edge plus ruby margin.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_START_EXTENDED_MESSAGE,
        symbol: "Graph91_90_StartExtendedMessage",
        parameters: GRAPH91_MESSAGE_EX_PARAMETERS,
        returns: "native procedure status 2",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "sub_484F30 -> sub_491D20: window, font set (+0x36C) fatal; DspMsgEx procedure, p3..p6 to +0x30/+0x38/+0x3C/+0x40.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_RENDER_WINDOW_TEXT,
        symbol: "Graph91_91_RenderWindowText",
        parameters: GRAPH91_RENDER_TEXT_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_484FF0 -> sub_4412D0 -> sub_42BB00: window and font fatal; glyphs are a renderer item.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_START_EXTENDED_MESSAGE_WITH_OPTION,
        symbol: "Graph91_92_StartExtendedMessageWithOption",
        parameters: GRAPH91_MESSAGE_EX_OPTION_PARAMETERS,
        returns: "native procedure status 2",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "sub_485080: 91:90 with p3 option; same checks.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_RENDER_WINDOW_TEXT_WITH_STYLE_MODE,
        symbol: "Graph91_93_RenderWindowTextWithStyleMode",
        parameters: GRAPH91_RENDER_TEXT_STYLE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_485150: 91:91 with style switch p3; same checks.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_UPDATE_TEXT_SUBSTITUTION,
        symbol: "Graph91_94_UpdateTextSubstitution",
        parameters: GRAPH91_TEXT_SUBSTITUTION_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_485210 -> sub_4637D0: null source clears, empty replacement removes, else registers.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_COUNT_TEXT_SUBSTITUTION_MATCHES,
        symbol: "Graph91_95_CollectRubyMatches",
        parameters: GRAPH91_TEXT_SUBSTITUTION_COUNT_PARAMETERS,
        returns: "i32 match count",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_485240 -> sub_434CC0: base\\reading records of matches, count.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_REGISTER_TEXT_SUBSTITUTION_RECORDS,
        symbol: "Graph91_96_RegisterTextSubstitutionRecords",
        parameters: GRAPH91_TEXT_SUBSTITUTION_RECORDS_PARAMETERS,
        returns: "i32 boolean",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_485270 -> sub_434DE0: base\\reading records up to newline; empty reading keeps the newline.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SET_PERSISTENT_TEXT_STYLE,
        symbol: "Graph91_97_SetPersistentTextStyle",
        parameters: GRAPH91_TEXT_STYLE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4852A0 -> sub_463820: font number resolved through sub_469110, p1..p4, -1, -1.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_CONFIGURE_TEXT_LAYOUT_GLOBALS,
        symbol: "Graph91_98_ConfigureTextLayoutGlobals",
        parameters: GRAPH91_TEXT_LAYOUT_GLOBAL_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_485300 -> sub_4346E0: ruby size rate 25..=100 and margin >= 0 fatal with target texts.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SET_TEXT_SCALE_DIVISOR,
        symbol: "Graph91_99_SetTextScaleDivisor",
        parameters: GRAPH91_TEXT_SCALE_DIVISOR_PARAMETERS,
        returns: "i32 boolean",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4853E0 -> sub_407B60: unsigned (d+0xFFFF)/d, 0 refused.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SET_TEXT_GLOBAL_PROPERTY,
        symbol: "Graph91_9A_SetTextGlobalProperty",
        parameters: GRAPH91_TEXT_GLOBAL_PROPERTY_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_485410 -> sub_434750: unknown function number and negative 0x80000001 parameter fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_MEASURE_TEXT,
        symbol: "Graph91_9B_MeasureText",
        parameters: GRAPH91_TEXT_MEASURE_PARAMETERS,
        returns: "i32 status and one BP output DWORD",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4854B0 -> sub_403BA0: p2 font number, p3 size, p4 width; returned statuses 0x80000001..3; cell-width arithmetic of sub_4352B0.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_DRAW_TEXT,
        symbol: "Graph91_9C_DrawText",
        parameters: GRAPH91_DRAW_TEXT_PARAMETERS,
        returns: "updated X/coordinate value",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_485530 -> sub_403B10: bitmap validator, font number, bitmap exists, size, width fatal; glyph layout is a renderer item.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_DRAW_TEXT_WITH_STYLE_MODE,
        symbol: "Graph91_9D_DrawTextWithStyleMode",
        parameters: GRAPH91_DRAW_TEXT_STYLE_PARAMETERS,
        returns: "updated X/coordinate value",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_485740: 91:9C with style switch p14.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_EXTRACT_TEXT_LABELS,
        symbol: "Graph91_9E_ExtractTextLabels",
        parameters: GRAPH91_TEXT_LABEL_PARAMETERS,
        returns: "i32 record count",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_485990 -> sub_4382A0: byte scan, \"<l>\" before \"<L>\", 95-byte payloads in 128-byte records, null output only counts.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_STRIP_TEXT_MARKUP,
        symbol: "Graph91_9F_StripTextMarkup",
        parameters: GRAPH91_STRIP_MARKUP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4859C0 -> sub_438430: byte-exact tag strip including the v2+2 '>' search.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_CREATE_EXTENDED_ICON_INPUT_PROCESSOR,
        symbol: "Graph91_B8_CreateExtendedIconInputProcessor",
        parameters: GRAPH91_EXTENDED_ICON_CREATE_PARAMETERS,
        returns: "input processor handle or 0",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4859F0 -> sub_46CB90(window, 1): DCIPIconEx built without checking the window, like 90:B8.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_CONFIGURE_EXTENDED_ICON_INPUT_PROCESSOR,
        symbol: "Graph91_BA_ConfigureExtendedIconInputProcessor",
        parameters: GRAPH91_EXTENDED_ICON_CONFIGURE_PARAMETERS,
        returns: "i32 status 0/1/2/3/4",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_485A20 -> sub_46D0C0: missing 1, not extended 4, descriptor statuses, 2/3 from sub_44AD40.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SET_EXTENDED_ICON_INPUT_ITEM_STATE,
        symbol: "Graph91_BB_SetExtendedIconInputItemState",
        parameters: GRAPH91_EXTENDED_ICON_ITEM_STATE_PARAMETERS,
        returns: "i32 status 0/1/2/3/4",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_485A60 -> sub_46D200 -> sub_44B8A0: missing 1, not extended 4, group count 2, item count 3.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_REGISTER_KEY_ASSIGNMENT_TABLE,
        symbol: "Graph91_BF_RegisterKeyAssignmentTable",
        parameters: GRAPH91_KEY_ASSIGNMENT_PARAMETERS,
        returns: "void; invalid IDs raise a script error",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_485AB0 -> sub_448010: assignments 4..=7, target text otherwise.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_GET_ACTIVE_KNOB_HANDLE,
        symbol: "Graph91_DB_GetActiveKnobHandle",
        parameters: NO_PARAMETERS,
        returns: "active 0xF0000000 Knob handle or 0",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_485B20 -> sub_463F00: handle in the active knob record (dword_565FDC) or 0.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_OPEN_DIRECTSHOW_MOVIE,
        symbol: "Graph91_F0_OpenDirectShowMovie",
        parameters: GRAPH91_MOVIE_OPEN_PARAMETERS,
        returns: "i32 status 0..4",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_485B40 -> sub_4902B0 -> sub_408880: bitmap, path, two options; DirectShow decode is the app's (host item); an unresolved path returns an uninitialized local in the target.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_START_DIRECTSHOW_MOVIE,
        symbol: "Graph91_F1_StartDirectShowMovie",
        parameters: GRAPH91_MOVIE_START_PARAMETERS,
        returns: "i32 status 0/1/4",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_485C00 -> sub_408420: invalid/empty slot 4, no movie 1, duration written on start.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_CLOSE_DIRECTSHOW_MOVIE,
        symbol: "Graph91_F2_CloseDirectShowMovie",
        parameters: GRAPH91_MOVIE_HANDLE_PARAMETERS,
        returns: "i32 status 0/1/4",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_485C70 -> sub_4084A0: invalid/empty slot 4, no movie 1.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_SET_DIRECTSHOW_MOVIE_PAUSED,
        symbol: "Graph91_F3_SetDirectShowMoviePaused",
        parameters: GRAPH91_MOVIE_PAUSE_PARAMETERS,
        returns: "i32 status 0/1/4",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_485CD0 -> sub_408540: invalid/empty slot 4, no movie 1; a stopped movie can be paused.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_CREATE_FLASH_CONTROL,
        symbol: "Graph91_F4_CreateFlashControl",
        parameters: GRAPH91_FLASH_CREATE_PARAMETERS,
        returns: "i32 status 0/1/2/4",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_485D40 -> sub_408AC0: Flash control (host item): bitmap, width, height, path, parameter; capture fail 1, allocation fail 4.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_START_FLASH_CONTROL,
        symbol: "Graph91_F5_StartFlashControl",
        parameters: GRAPH91_MOVIE_HANDLE_PARAMETERS,
        returns: "i32 status 0/1/2/4",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_485E00 -> sub_408B70: Flash start (host item).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_CAPTURE_AND_RELEASE_FLASH_CONTROL,
        symbol: "Graph91_F6_CaptureAndReleaseFlashControl",
        parameters: GRAPH91_MOVIE_HANDLE_PARAMETERS,
        returns: "i32 status 0/1/4",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_485E80 -> sub_408BD0: Flash capture and release (host item).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH91_GET_MOVIE_POSITION,
        symbol: "Graph91_F7_GetMoviePosition",
        parameters: GRAPH91_MOVIE_POSITION_PARAMETERS,
        returns: "i32 boolean",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_485EF0 -> sub_407FA0: invalid/empty slot false; no movie or stopped movie writes -1 and returns true.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH_SET_GLYPH_REVEAL_DELAY,
        symbol: "Graph90_94_SetGlyphRevealDelay",
        parameters: GLYPH_DELAY_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47EC00 -> sub_433890: dword_507660 glyph delay.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH_SET_TEXT_REVEAL_ANIMATION,
        symbol: "Graph90_95_SetTextRevealAnimation",
        parameters: TEXT_ANIMATION_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47EC20 -> sub_4338A0: division count 1..=256 (fatal) and step delay.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH_SET_TEXT_SETTLE_ANIMATION,
        symbol: "Graph90_96_SetTextSettleAnimation",
        parameters: TEXT_ANIMATION_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47EC60 -> sub_4338C0: division count 1..=256 (fatal) and step delay.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH_SET_TEXT_AUTO_ADVANCE,
        symbol: "Graph90_97_SetTextAutoAdvance",
        parameters: TEXT_ENABLE_DELAY_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47ECA0 -> sub_4338E0: auto advance pair.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH_SET_MESSAGE_START_DELAY,
        symbol: "Graph90_9B_SetMessageStartDelay",
        parameters: TEXT_ENABLE_DELAY_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47EDB0 -> sub_433880: message start delay pair.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH_SET_MESSAGE_INPUT_FORCES_COMPLETION,
        symbol: "Graph90_9F_SetMessageInputForcesCompletion",
        parameters: BOOLEAN_PARAMETER,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_47EF60 -> sub_433140: dword_565BB0.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_SET_TEXT_OBJECT_VALUE,
        symbol: "Graph92_88_SetTextObjectValue",
        parameters: GRAPH92_TEXT_OBJECT_VALUE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_486BD0 -> sub_42B7A0: window fatal; window+0x178.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_COMPOSITE_RESOURCE_INTO_TEXT_OBJECT,
        symbol: "Graph92_89_CompositeResourceIntoTextObject",
        parameters: GRAPH92_TEXT_OBJECT_COMPOSITE_PARAMETERS,
        returns: "void or script error",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_486C10 -> sub_42B7F0: validators, window, bitmap, pixel mode vs screen; non-window bitmap handles (older engines) keep the bitmap path.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_SET_TEXT_OBJECT_AUXILIARY_VALUE,
        symbol: "Graph92_8A_SetTextObjectAuxiliaryValue",
        parameters: GRAPH92_TEXT_OBJECT_VALUE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_486D20 -> sub_42B7B0: window fatal; buffer fill.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_SET_TEXT_OBJECT_UPDATE_FLAG,
        symbol: "Graph92_8C_SetTextObjectUpdateFlag",
        parameters: GRAPH92_TEXT_OBJECT_VALUE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_486D60 -> sub_42B930: window fatal; window+0x198.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_APPLY_EFFECT_RESOURCE,
        symbol: "Graph92_8D_ApplyEffectResource",
        parameters: GRAPH92_TEXT_OBJECT_COMPOSITE_PARAMETERS,
        returns: "void or script error",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_486DA0 -> sub_42B950: as 92:89.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_RESET_TEXT_OBJECT,
        symbol: "Graph92_8E_ResetTextObject",
        parameters: GRAPH92_RESET_TEXT_OBJECT_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_486EB0 -> sub_441340: window fatal; cursor reset and buffer clear.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_START_STYLED_MESSAGE,
        symbol: "Graph92_90_StartStyledMessage",
        parameters: GRAPH92_START_STYLED_MESSAGE_PARAMETERS,
        returns: "native procedure status 2",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "sub_486EE0 -> sub_491D20 (variant 1 selects the vertical class): window and font fatal for window handles.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_DRAW_FORMATTED_TEXT,
        symbol: "Graph92_91_DrawFormattedText",
        parameters: GRAPH92_DRAW_FORMATTED_TEXT_PARAMETERS,
        returns: "void or script error",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_487000 -> sub_463600: window and font fatal for window handles.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_CONFIGURE_TEXT_BITMAP_SLOT,
        symbol: "Graph92_98_ConfigureTextBitmapSlot",
        parameters: GRAPH92_CONFIGURE_TEXT_BITMAP_SLOT_PARAMETERS,
        returns: "void or script error",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_487150 -> sub_433210: code 0xFF01..=0xFFFF, -1 clears, bitmap, nonzero size with target texts.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_GET_TEXT_OUTPUT_PAIR,
        symbol: "Graph92_9B_GetTextOutputPair",
        parameters: GRAPH92_GET_TEXT_OUTPUT_PAIR_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_487260 -> sub_4347B0: 256 pair, 257 colour (dword_565D4C, colour tags not followed), else fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_RENDER_TEXT,
        symbol: "Graph92_9C_RenderText",
        parameters: GRAPH92_RENDER_TEXT_PARAMETERS,
        returns: "updated x/cursor i32",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4872D0 -> sub_403B10: bitmap validator, font, bitmap, size, width texts; glyphs renderer item.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_CONFIGURE_FONT_OVERRIDE,
        symbol: "Graph92_9D_ConfigureFontOverride",
        parameters: GRAPH92_CONFIGURE_FONT_OVERRIDE_PARAMETERS,
        returns: "status 0..=3 or -1",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_487550 -> sub_438360: font number face; nonzero height outside 4..=200 is 2, width 25..=200 is 3; nothing stored on refusal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_DRAIN_TEXT_FRAGMENT_RECORDS,
        symbol: "Graph92_9E_DrainTextFragmentRecords",
        parameters: GRAPH92_DRAIN_TEXT_FRAGMENT_RECORDS_PARAMETERS,
        returns: "record count 0..=16",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_487630 -> sub_438270: fragment records copied and cleared.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_SET_TEXT_RENDER_OVERRIDE,
        symbol: "Graph92_9F_SetTextRenderOverride",
        parameters: GRAPH92_SET_TEXT_RENDER_OVERRIDE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_487660 -> sub_438410: dword_507650.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_OPEN_GLOBAL_DIRECTSHOW_MOVIE,
        symbol: "Graph92_F0_OpenGlobalDirectShowMovie",
        parameters: GRAPH92_OPEN_GLOBAL_MOVIE_PARAMETERS,
        returns: "duration milliseconds or 0",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_487680: zero size fatal with target text; DirectShow host item.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_LOAD_BURIKO_MOVIE_RESOURCE,
        symbol: "Graph92_F1_LoadBurikoMovieResource",
        parameters: GRAPH92_LOAD_BURIKO_MOVIE_PARAMETERS,
        returns: "deferred loader status",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "sub_486C40 pops mode/resource/archive/metadata_out/handle_out. Mode zero installs DCProcLoadBurikoMV; nonzero installs DCProcLoadBMVHeader. The VM owns the handle and five-DWORD metadata writes.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_OPEN_BITMAP_DIRECTSHOW_MOVIE,
        symbol: "Graph92_F2_OpenBitmapDirectShowMovie",
        parameters: GRAPH92_OPEN_BITMAP_MOVIE_PARAMETERS,
        returns: "status 0/1/2/3/4",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_487830 -> sub_4902B0 with archive: host DirectShow item; statuses as 91:F0.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_SEEK_BITMAP_DIRECTSHOW_MOVIE,
        symbol: "Graph92_F4_SeekBitmapDirectShowMovie",
        parameters: GRAPH92_SEEK_BITMAP_MOVIE_PARAMETERS,
        returns: "status 0/1/4/5",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_487910 -> sub_408590: slot 4, no movie 1, seek failure 5.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_GET_BITMAP_DIRECTSHOW_MOVIE_POSITION,
        symbol: "Graph92_F5_GetBitmapDirectShowMoviePosition",
        parameters: GRAPH92_GET_BITMAP_MOVIE_POSITION_PARAMETERS,
        returns: "status 0/1/4",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4879A0 -> sub_4085F0: slot 4, no movie 1.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH92_SET_BITMAP_DIRECTSHOW_MOVIE_VOLUME,
        symbol: "Graph92_F6_SetBitmapDirectShowMovieVolume",
        parameters: GRAPH92_SET_BITMAP_MOVIE_VOLUME_PARAMETERS,
        returns: "status 0/1/3/4",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_487A10 -> sub_408640: slot 4, no movie 1, volume 3.",
    },
    // SystemA0 target sound dispatcher.
    NativeOpcodeSpec {
        opcode: opcodes::SOUND_QUERY_CHANNEL_COUNT,
        symbol: "SoundA0_00_QueryChannelCount",
        parameters: NO_PARAMETERS,
        returns: "20",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_487010 returns the target engine's fixed software-audio channel count.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SOUND_SET_BGM_PRIMARY_VOLUME,
        symbol: "SoundA0_08_SetBgmPrimaryVolume",
        parameters: SOUND_CHANNEL_VOLUME_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_487030 -> sub_493B60 updates the first independent BGM gain bank.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SOUND_SET_SE_PRIMARY_VOLUME,
        symbol: "SoundA0_09_SetSePrimaryVolume",
        parameters: SOUND_CHANNEL_VOLUME_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_487070 -> sub_493B90 updates the first independent SE gain bank.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SOUND_LOAD_BGM_FILE,
        symbol: "SoundA0_10_LoadBgmFile",
        parameters: SOUND_BGM_FILE_LOAD_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_487BB0 -> sub_4946F0: volume, BGM channel, then missing (12) and non-BW (14) files are fatal with target texts.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SOUND_LOAD_BGM_ARCHIVE,
        symbol: "SoundA0_11_LoadBgmArchive",
        parameters: SOUND_BGM_ARCHIVE_LOAD_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_487C80 -> sub_4948A0: pan, volume, channel, then 12/14 with \"[ archive : file ]\".",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SOUND_LOAD_BGM_PAIR,
        symbol: "SoundA0_12_LoadBgmPair",
        parameters: SOUND_BGM_PAIR_LOAD_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_487D80 -> sub_494BC0: pan, volume, channel, then 12/14 with \"[ archive : intro / loop ]\".",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SOUND_CONTROL_BGM,
        symbol: "SoundA0_14_ControlBgm",
        parameters: SOUND_BGM_CONTROL_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_487EC0 -> sub_4A3CC0(channel, value == 0).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SOUND_QUERY_BGM_STATE,
        symbol: "SoundA0_15_QueryBgmState",
        parameters: SOUND_BGM_QUERY_PARAMETERS,
        returns: "status",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_487F00 -> sub_494DA0: channel validator; status pushed.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SOUND_SET_BGM_VOLUME,
        symbol: "SoundA0_16_SetBgmVolume",
        parameters: SOUND_BGM_FADE_VOLUME_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_487F50 -> sub_4A3D50: volume then channel; linear ramp.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SOUND_SET_BGM_PAN,
        symbol: "SoundA0_17_SetBgmPan",
        parameters: SOUND_CHANNEL_PAN_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_487FA0 -> sub_4A3F00: pan then channel.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SOUND_FADE_BGM_TO_FULL,
        symbol: "SoundA0_18_FadeBgmToFull",
        parameters: SOUND_CHANNEL_DURATION_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_487FE0 -> sub_4A3BA0: channel; fade to full.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SOUND_FADE_BGM_TO_SILENCE,
        symbol: "SoundA0_19_FadeBgmToSilence",
        parameters: SOUND_CHANNEL_DURATION_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_488020 -> sub_4A3B10: channel; fade to silence.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SOUND_SET_BGM_SECONDARY_VOLUME,
        symbol: "SoundA0_1C_SetBgmSecondaryVolume",
        parameters: SOUND_CHANNEL_VOLUME_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_487560 -> sub_4A2BB0 updates the second independent BGM gain bank.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SOUND_LOAD_SE,
        symbol: "SoundA0_20_LoadSe",
        parameters: SOUND_SE_LOAD_PARAMETERS,
        returns: "deferred load status",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "sub_4880A0 -> CProcLoadSound(channel, archive, file, 0, 1.0, 1.0): SE channel, non-BW data fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SOUND_LOAD_SE_SCALED,
        symbol: "SoundA0_21_LoadSeScaled",
        parameters: SOUND_SE_LOAD_SCALED_PARAMETERS,
        returns: "deferred load status",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "sub_488150: as 20 with start parameter and 16.16 gain.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SOUND_RELEASE_SE,
        symbol: "SoundA0_22_ReleaseSe",
        parameters: SOUND_CHANNEL_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_488240 -> sub_494E70: SE channel; release.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SOUND_LOAD_SE_DOUBLE_RATE,
        symbol: "SoundA0_23_LoadSeDoubleRate",
        parameters: SOUND_SE_LOAD_SCALED_PARAMETERS,
        returns: "deferred load status",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "sub_488270: as 21 with rate 2.0.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SOUND_PLAY_SE,
        symbol: "SoundA0_24_PlaySe",
        parameters: SOUND_SE_PLAY_PARAMETERS,
        returns: "playback position/status",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_488360: pan, volume, SE channel; length pushed when sub_494E90 gives 0 or 20, else 0.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SOUND_STOP_SE,
        symbol: "SoundA0_25_StopSe",
        parameters: SOUND_CHANNEL_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4883F0 -> sub_494EB0: SE channel; stop.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SOUND_FADE_SE_TO_SILENCE,
        symbol: "SoundA0_26_FadeSeToSilence",
        parameters: SOUND_CHANNEL_DURATION_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_488420 -> sub_494EC0: SE channel; fade.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SOUND_LOAD_SE_CUSTOM_RATE,
        symbol: "SoundA0_27_LoadSeCustomRate",
        parameters: SOUND_SE_LOAD_CUSTOM_PARAMETERS,
        returns: "deferred load status",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "sub_488460: as 21 with 16.16 gain and rate.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SOUND_REGISTER_SE_MEMORY,
        symbol: "SoundA0_28_RegisterSeMemory",
        parameters: SOUND_SE_REGISTER_PARAMETERS,
        returns: "deferred registration status",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "sub_488570 -> sub_452980: SE channel; memory registration procedure.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SOUND_SET_SE_SECONDARY_VOLUME,
        symbol: "SoundA0_2C_SetSeSecondaryVolume",
        parameters: SOUND_CHANNEL_VOLUME_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_487B60 -> sub_4A2890 updates the second independent SE gain bank.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SOUND_GET_SE_POSITION,
        symbol: "SoundA0_2F_GetSeLength",
        parameters: SOUND_POSITION_QUERY_PARAMETERS,
        returns: "position/status",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4886A0 -> sub_494ED0: SE channel; length.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SOUND_OPEN_CD_AUDIO,
        symbol: "SoundA0_80_OpenCdAudio",
        parameters: NO_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4886E0 -> sub_48E130: MCI CD open (host item).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SOUND_CLOSE_CD_AUDIO,
        symbol: "SoundA0_81_CloseCdAudio",
        parameters: NO_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4886F0 -> sub_48E2B0: MCI close (host item).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SOUND_PLAY_CD_TRACK,
        symbol: "SoundA0_84_PlayCdTrack",
        parameters: SOUND_CD_PLAY_PARAMETERS,
        returns: "status",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_488700 -> sub_48E340: TMSF track to track+1, notify flag; success pushed (host item).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SOUND_STOP_CD_AUDIO,
        symbol: "SoundA0_85_StopCdAudio",
        parameters: NO_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_488740 -> sub_48E3E0: MCI stop (host item).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SOUND_QUERY_CD_MODE,
        symbol: "SoundA0_86_QueryCdMode",
        parameters: SOUND_CD_QUERY_PARAMETERS,
        returns: "zero or one",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_487C50 -> sub_48D910 maps MCI modes 524..530 to target values 0,3,2,6,1,4,5; VM mediation writes the virtual device mode and returns one only while the device is open.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::SOUND_PLAY_WAVE_ASYNC,
        symbol: "SoundA0_C0_PlayWaveAsync",
        parameters: SOUND_WAVE_PLAY_PARAMETERS,
        returns: "BOOL",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_488780 -> sub_494FC0: PlaySoundA(dir + file, 0x22003) result pushed (host item; loose file per ADR 0004).",
    },
    // SystemB0 target-confirmed user/window/input/font handlers.
    NativeOpcodeSpec {
        opcode: opcodes::USER_DRAW_BITMAP_TO_WINDOW,
        symbol: "UserB0_00_DrawBitmapToWindow",
        parameters: USER_DRAW_BITMAP_TO_WINDOW_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_478B30 -> sub_461F20: bitmap validator, missing bitmap fatal; drawn to the parent window DC (host item).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_CENTER_MAIN_WINDOW,
        symbol: "UserB0_02_CenterMainWindow",
        parameters: NO_PARAMETERS,
        returns: "boolean success",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_478BC0 -> sub_461BF0: centres a windowed main window (host item).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_SET_MAIN_WINDOW_POSITION,
        symbol: "UserB0_03_SetMainWindowPosition",
        parameters: USER_SET_MAIN_WINDOW_POSITION_PARAMETERS,
        returns: "boolean accepted",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_478BE0: monitor/work-area check then SetWindowPos (host item).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_BIND_CURSOR_OBJECT,
        symbol: "UserB0_04_BindCursorObject",
        parameters: USER_BIND_CURSOR_OBJECT_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_478C70 -> sub_48F6D0: unknown object fatal; 0 unbinds.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_SET_CURSOR_IDLE_TIMEOUT,
        symbol: "UserB0_05_SetCursorIdleTimeout",
        parameters: USER_SET_CURSOR_IDLE_TIMEOUT_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_478CC0 -> sub_48F350: cursor idle timeout.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_QUERY_CURSOR_VISIBLE,
        symbol: "UserB0_06_QueryCursorVisible",
        parameters: NO_PARAMETERS,
        returns: "previous cursor visibility latch",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_478CE0 -> sub_48F920: cursor visible latch.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_SHAKE_SCREEN,
        symbol: "UserB0_08_ShakeScreen",
        parameters: USER_SHAKE_SCREEN_PARAMETERS,
        returns: "no deferred values",
        scheduling: NativeSchedulingEffect::WaitProcedure,
        notes: "sub_478D00 -> sub_43D0A0: pattern < 4, frequency >= 1, repeats >= 1, frame rate >= frequency with target texts.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_CREATE_DEBUG_WINDOW,
        symbol: "UserB0_10_CreateDebugWindow",
        parameters: USER_CREATE_DEBUG_WINDOW_PARAMETERS,
        returns: "tagged debug-window handle",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_478ED0 -> sub_42FEC0: 32..=2048 size, eight slots, both fatal with target texts.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_CLOSE_DEBUG_WINDOW,
        symbol: "UserB0_11_CloseDebugWindow",
        parameters: USER_CLOSE_DEBUG_WINDOW_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_478FA0 -> sub_430000: unknown handle fatal (target text repeats チャイルド).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_SET_DEBUG_WINDOW_VISIBLE,
        symbol: "UserB0_14_SetDebugWindowVisible",
        parameters: USER_SET_DEBUG_WINDOW_VISIBLE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_478FD0 -> sub_430030: unknown handle fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_SET_DEBUG_WINDOW_TITLE,
        symbol: "UserB0_15_SetDebugWindowTitle",
        parameters: USER_SET_DEBUG_WINDOW_TITLE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479010 -> sub_430110: unknown handle fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_MOVE_DEBUG_WINDOW,
        symbol: "UserB0_16_MoveDebugWindow",
        parameters: USER_MOVE_DEBUG_WINDOW_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479050 -> sub_430080: unknown handle fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_GET_DEBUG_WINDOW_POSITION,
        symbol: "UserB0_17_GetDebugWindowPosition",
        parameters: USER_GET_DEBUG_WINDOW_POSITION_PARAMETERS,
        returns: "x then y",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4790A0 -> sub_4300C0: unknown handle fatal; pushes x, y.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_CLEAR_DEBUG_WINDOW,
        symbol: "UserB0_18_ClearDebugWindow",
        parameters: USER_CLEAR_DEBUG_WINDOW_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4790F0 -> sub_430140: unknown handle fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_DRAW_DEBUG_BITMAP,
        symbol: "UserB0_19_DrawDebugBitmap",
        parameters: USER_DRAW_DEBUG_BITMAP_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479130 -> sub_4301A0: bitmap/mode/level validators, handle, bitmap, pixel mode, outside client, target texts.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_DRAW_DEBUG_TEXT,
        symbol: "UserB0_1A_DrawDebugText",
        parameters: USER_DRAW_DEBUG_TEXT_PARAMETERS,
        returns: "rendered text extent",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479280 -> sub_4302A0: font validator, handle, height 4..=200, width 25..=200 (p6); advance pushed; glyphs host item.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_SET_DEBUG_WINDOW_CLOSE_MESSAGE,
        symbol: "UserB0_1C_SetDebugWindowCloseMessage",
        parameters: USER_SET_DEBUG_WINDOW_CLOSE_MESSAGE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479420 -> sub_430390: unknown handle fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_CREATE_EDIT_CONTROL,
        symbol: "UserB0_20_CreateEditControl",
        parameters: USER_CREATE_EDIT_CONTROL_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479460 -> sub_464480: size within screen, font, height 8..=64, 1..=256 characters with target texts; position unchecked.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_DESTROY_EDIT_CONTROL,
        symbol: "UserB0_21_DestroyEditControl",
        parameters: NO_PARAMETERS,
        returns: "boolean was active",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4795D0 -> sub_4646F0: destroys, pushes 0.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_SET_EDIT_FONT_SCALE,
        symbol: "UserB0_22_SetEditFontScale",
        parameters: USER_SET_EDIT_FONT_SCALE_PARAMETERS,
        returns: "boolean accepted",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4795F0 -> sub_464750: 25..=200 accepted.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_QUERY_EDIT_ACTIVE,
        symbol: "UserB0_23_QueryEditActive",
        parameters: NO_PARAMETERS,
        returns: "boolean active",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479620: shown flag dword_565FF8.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_SET_EDIT_VISIBLE,
        symbol: "UserB0_24_SetEditVisible",
        parameters: USER_SET_EDIT_VISIBLE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479640 -> sub_464770: show/hide an existing control.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_SET_EDIT_COLOR,
        symbol: "UserB0_25_SetEditColor",
        parameters: USER_SET_EDIT_COLOR_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479660 -> sub_4647F0: RGB to BGR colour.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_SET_EDIT_TEXT,
        symbol: "UserB0_26_SetEditText",
        parameters: USER_SET_EDIT_TEXT_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479680 -> sub_464860: initial text buffer, not cut by the limit.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_GET_EDIT_TEXT,
        symbol: "UserB0_27_GetEditText",
        parameters: USER_GET_EDIT_TEXT_PARAMETERS,
        returns: "bytes written",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4796A0 -> sub_464880: GetWindowText (256).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_SET_EDIT_HIDE_ON_ENTER,
        symbol: "UserB0_28_SetEditHideOnEnter",
        parameters: USER_SET_EDIT_HIDE_ON_ENTER_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4796D0 -> sub_4648B0: hide-on-enter flag.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_SET_EDIT_PRINTABLE_INPUT,
        symbol: "UserB0_29_SetEditPrintableInput",
        parameters: USER_SET_EDIT_PRINTABLE_INPUT_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4796F0 -> sub_4648C0: printable-input flag.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_SHOW_MESSAGE,
        symbol: "UserB0_80_ShowMessage",
        parameters: USER_SHOW_MESSAGE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479710 -> sub_46C1E0(MB_ICONINFORMATION) (host dialog).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_SHOW_YES_NO_MESSAGE,
        symbol: "UserB0_81_ShowYesNoMessage",
        parameters: USER_SHOW_YES_NO_MESSAGE_PARAMETERS,
        returns: "boolean yes",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479740: MB_YESNO|ICONQUESTION, DEFBUTTON2 when p1 == 0; IDYES pushed (host dialog).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_SHOW_TYPED_MESSAGE,
        symbol: "UserB0_82_ShowTypedMessage",
        parameters: USER_SHOW_TYPED_MESSAGE_PARAMETERS,
        returns: "boolean accepted",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4797A0: type 1 OK/Cancel (IDOK), else Yes/No (host dialog).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_SET_MESSAGE_TITLE,
        symbol: "UserB0_83_SetMessageTitle",
        parameters: USER_SET_MESSAGE_TITLE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479850 -> sub_46C190: caption (null restores default).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_SHOW_INPUT_DIALOG,
        symbol: "UserB0_84_ShowInputDialog",
        parameters: USER_SHOW_INPUT_DIALOG_PARAMETERS,
        returns: "dialog result",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479870 -> sub_45D070 (host dialog, 4 arguments).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_SHOW_MULTI_FIELD_DIALOG,
        symbol: "UserB0_85_ShowMultiFieldDialog",
        parameters: USER_SHOW_MULTI_FIELD_DIALOG_PARAMETERS,
        returns: "dialog result",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4798C0 -> sub_45D420 (host dialog, 9 arguments).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_SHOW_SELECTION_DIALOG,
        symbol: "UserB0_86_ShowSelectionDialog",
        parameters: USER_SHOW_SELECTION_DIALOG_PARAMETERS,
        returns: "dialog result",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479960 -> sub_45D900 (host dialog, 5 arguments).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_SHOW_EXTENDED_DIALOG,
        symbol: "UserB0_87_ShowExtendedDialog",
        parameters: USER_SHOW_EXTENDED_DIALOG_PARAMETERS,
        returns: "dialog result",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4799C0 -> sub_45D420 (host dialog, 12 arguments).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_SHOW_PATH_DIALOG,
        symbol: "UserB0_8C_ShowPathDialog",
        parameters: USER_SHOW_PATH_DIALOG_PARAMETERS,
        returns: "dialog result",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479A80 -> sub_45E2B0 (host dialog, 4 arguments).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_SHOW_SIX_FIELD_DIALOG,
        symbol: "UserB0_8F_ShowSixFieldDialog",
        parameters: USER_SHOW_SIX_FIELD_DIALOG_PARAMETERS,
        returns: "dialog result",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479AD0 -> sub_45E000 (host dialog, 6 arguments).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_CREATE_MODELESS_DIALOG,
        symbol: "UserB0_A0_CreateModelessDialog",
        parameters: USER_CREATE_MODELESS_DIALOG_PARAMETERS,
        returns: "boolean success",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479B40 -> sub_45E870: success pushed as status == 0 (host dialog).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_CLOSE_MODELESS_DIALOG,
        symbol: "UserB0_A1_CloseModelessDialog",
        parameters: USER_CLOSE_MODELESS_DIALOG_PARAMETERS,
        returns: "boolean success",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479B90 -> sub_45E910 (host dialog).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_SET_MODELESS_DIALOG_VISIBLE,
        symbol: "UserB0_A2_SetModelessDialogVisible",
        parameters: USER_SET_MODELESS_DIALOG_VISIBLE_PARAMETERS,
        returns: "boolean success",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479BC0 -> sub_45E980 (host dialog).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_POLL_MODELESS_DIALOG,
        symbol: "UserB0_A3_PollModelessDialog",
        parameters: USER_POLL_MODELESS_DIALOG_PARAMETERS,
        returns: "0 event, 1 empty, -1 invalid, -2 error",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479C00 -> sub_45E3D0: 0 event written, 1, -1 (0x80000000), else -2.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_INTERN_FONT_NAME,
        symbol: "UserB0_C0_InternFontName",
        parameters: USER_INTERN_FONT_NAME_PARAMETERS,
        returns: "stable font id",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479C90 -> sub_468FD0: intern name, no charset change.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_INTERN_FONT_NAME_WITH_OPTION,
        symbol: "UserB0_C1_InternFontNameWithOption",
        parameters: USER_INTERN_FONT_NAME_WITH_OPTION_PARAMETERS,
        returns: "stable font id",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479CC0 -> sub_468FD0: option 0 charset 128, 1 charset 0.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_REGISTER_FONT_RESOURCE,
        symbol: "UserB0_C2_RegisterFontResource",
        parameters: USER_REGISTER_FONT_RESOURCE_PARAMETERS,
        returns: "registration status",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479CF0 -> sub_469130(name, 0): AddFontResource of a loose file (host item).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_REGISTER_ARCHIVE_FONT_RESOURCE,
        symbol: "UserB0_C3_RegisterArchiveFontResource",
        parameters: USER_REGISTER_ARCHIVE_FONT_RESOURCE_PARAMETERS,
        returns: "registration status",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479D20 -> sub_469130(name, archive): AddFontMemResourceEx (host item).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_QUERY_FONT_AVAILABLE,
        symbol: "UserB0_C4_QueryFontAvailable",
        parameters: USER_QUERY_FONT_AVAILABLE_PARAMETERS,
        returns: "availability/count",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479D60 -> sub_42DEB0 (host font query).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_QUERY_FONT_CAPABILITY,
        symbol: "UserB0_C6_QueryFontCapability",
        parameters: USER_QUERY_FONT_CAPABILITY_PARAMETERS,
        returns: "boolean capability",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479D90 -> sub_42DF30 (host font query).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_SET_FONT_ALIAS,
        symbol: "UserB0_C7_SetFontAlias",
        parameters: USER_SET_FONT_ALIAS_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479DC0 -> sub_42DBF0: alias.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER_SET_DESKTOP_WALLPAPER,
        symbol: "UserB0_F0_SetDesktopWallpaper",
        parameters: USER_SET_DESKTOP_WALLPAPER_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_479DF0 -> sub_4998A0: desktop wallpaper (host item).",
    },
    // SystemC0 particle, rain, spline, and BWEF handlers.
    NativeOpcodeSpec {
        opcode: opcodes::USER2_CREATE_PARTICLE_SCREEN,
        symbol: "UserC0_00_CreateParticleScreen",
        parameters: USER_C0_00_PARAMETERS,
        returns: "particle-screen handle",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_475F20 -> sub_4414F0: eight screens first, then the size; target texts.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_RELEASE_PARTICLE_SCREEN,
        symbol: "UserC0_01_ReleaseParticleScreen",
        parameters: USER_C0_01_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_475FC0: unknown screen fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_SET_PARTICLE_SCREEN_ENABLED,
        symbol: "UserC0_04_SetParticleScreenEnabled",
        parameters: USER_C0_04_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_475FF0: unknown screen fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_CONFIGURE_PARTICLE_SCREEN_DISPLAY,
        symbol: "UserC0_05_ConfigureParticleScreenDisplay",
        parameters: USER_C0_05_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_476030: priority, level, mode validators, then the screen.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_CONFIGURE_PARTICLE_FRAME_TABLES,
        symbol: "UserC0_06_ConfigureParticleFrameTables",
        parameters: USER_C0_06_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4760C0 -> sub_4417A0 -> sub_424A50/sub_425010: screen 255, 1..=16 layers 7, relative priority + screen priority <= 0xFFFF 8; one layer defaults to 0x7FFF.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_COMMIT_PARTICLE_SCREEN,
        symbol: "UserC0_08_CommitParticleScreen",
        parameters: USER_C0_08_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_476170: unknown screen fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_SET_PARTICLE_AUTO_UPDATE,
        symbol: "UserC0_09_SetParticleAutoUpdate",
        parameters: USER_C0_09_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4761A0: unknown screen fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_SET_PARTICLE_CAPACITY,
        symbol: "UserC0_0A_SetParticleCapacity",
        parameters: USER_C0_0A_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4761E0: unknown screen fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_CONFIGURE_PARTICLE_EMITTER_TRANSFORM,
        symbol: "UserC0_0B_ConfigureParticleEmitterTransform",
        parameters: USER_C0_0B_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_476220 -> sub_4418A0: screen, then the scale (3).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_SET_PARTICLE_EMISSION_PERCENT,
        symbol: "UserC0_0C_SetParticleEmissionPercent",
        parameters: USER_C0_0C_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_476330 -> sub_441900: screen, then the interval (4).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_ADVANCE_PARTICLE_SCREEN,
        symbol: "UserC0_0D_AdvanceParticleScreen",
        parameters: USER_C0_0D_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4763B0: unknown screen fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_RESET_PARTICLE_SCREEN,
        symbol: "UserC0_0F_ResetParticleScreen",
        parameters: USER_C0_0F_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4763F0: unknown screen fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_CONFIGURE_PARTICLE_EMITTER,
        symbol: "UserC0_10_ConfigureParticleEmitter",
        parameters: USER_C0_10_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_476420: unknown screen fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_CONFIGURE_PARTICLE_ANIMATION_BANK,
        symbol: "UserC0_18_ConfigureParticleAnimationBank",
        parameters: USER_C0_18_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4764F0 -> sub_491900: bitmaps exist, type, pattern, count <= 32, one RGB/ARGB size, target texts.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_LOAD_PARTICLE_ANIMATION_FRAMES,
        symbol: "UserC0_1A_LoadParticleAnimationFrames",
        parameters: USER_C0_1A_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_476640 -> sub_491A60: mixing ratio, bitmaps, type 1, pattern, count, suitability.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_COMMIT_PARTICLE_ANIMATION_FRAMES,
        symbol: "UserC0_1B_CommitParticleAnimationFrames",
        parameters: USER_C0_1B_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4767B0 -> sub_491BC0: mixing ratio, type, pattern, secondary image present.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_SET_PARTICLE_INTERPOLATION_MODE,
        symbol: "UserC0_1F_SetParticleInterpolationMode",
        parameters: USER_C0_1F_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_476890 -> sub_491BE0: control mode <= 2.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_SET_PARTICLE_STYLE0,
        symbol: "UserC0_20_SetParticleStyle0",
        parameters: USER_C0_20_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4768F0 -> sub_441A00: screen, pattern (10), maximum count (6).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_DEFINE_PARTICLE_STYLE0,
        symbol: "UserC0_24_DefineParticleStyle0",
        parameters: USER_C0_24_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4769B0 -> sub_491900(0, ...): bitmap, pattern, particle-suitable bitmap.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_CONFIGURE_PARTICLE_STYLE0,
        symbol: "UserC0_25_ConfigureParticleStyle0",
        parameters: USER_C0_25_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_476A80 -> sub_491C20: pattern < 64.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_SET_PARTICLE_STYLE1,
        symbol: "UserC0_28_SetParticleStyle1",
        parameters: USER_C0_28_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_476BB0 -> sub_441A60: screen, pattern, maximum count.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_CONFIGURE_PARTICLE_STYLE1,
        symbol: "UserC0_29_ConfigureParticleStyle1",
        parameters: USER_C0_29_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_476C70 -> sub_441AC0: screen, pattern.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_DEFINE_PARTICLE_STYLE1,
        symbol: "UserC0_2C_DefineParticleStyle1",
        parameters: USER_C0_2C_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_476E00 -> sub_491900(1, ...): as 24.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_CONFIGURE_PARTICLE_ADVANCED_STYLE,
        symbol: "UserC0_2D_ConfigureParticleAdvancedStyle",
        parameters: USER_C0_2D_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_476ED0 -> sub_491CD0: pattern < 64.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_CREATE_RAIN_SCREEN,
        symbol: "UserC0_40_CreateRainScreen",
        parameters: USER_C0_40_PARAMETERS,
        returns: "rain-screen handle",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_477090 -> sub_441C70: eight screens, then the size.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_RELEASE_RAIN_SCREEN,
        symbol: "UserC0_41_ReleaseRainScreen",
        parameters: USER_C0_41_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_477130: unknown screen fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_INITIALIZE_RAIN_SCREEN,
        symbol: "UserC0_42_InitializeRainScreen",
        parameters: USER_C0_42_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_477160: unknown screen fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_SET_RAIN_TEXTURE,
        symbol: "UserC0_43_SetRainTexture",
        parameters: USER_C0_43_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4771A0 -> sub_4259A0: screen, -1 clears, missing (4), not a screen-sized grayscale mask (5).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_SET_RAIN_ENABLED,
        symbol: "UserC0_44_SetRainEnabled",
        parameters: USER_C0_44_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_477240: unknown screen fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_CONFIGURE_RAIN_DISPLAY,
        symbol: "UserC0_45_ConfigureRainDisplay",
        parameters: USER_C0_45_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_477280: priority (p5), level (p4), blend mode (p3) validators then the screen; (x, y, mode, level, priority).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_SET_RAIN_VOLUME_BOUNDS,
        symbol: "UserC0_46_SetRainVolumeBounds",
        parameters: USER_C0_46_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_477320: unknown screen fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_SET_RAIN_DROP_WIDTH,
        symbol: "UserC0_47_SetRainDropWidth",
        parameters: USER_C0_47_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4773B0 -> sub_425550: screen, zero fall distance per stage.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_SET_RAIN_DROP_HEIGHT,
        symbol: "UserC0_48_SetRainDropHeight",
        parameters: USER_C0_48_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_477430 -> sub_4255A0: screen, zero rain length.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_SET_RAIN_COLOR,
        symbol: "UserC0_49_SetRainColor",
        parameters: USER_C0_49_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4774B0: unknown screen fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_SET_RAIN_DENSITY,
        symbol: "UserC0_4A_SetRainDensity",
        parameters: USER_C0_4A_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4774F0: unknown screen fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_SET_RAIN_SPEED,
        symbol: "UserC0_4B_SetRainSpeed",
        parameters: USER_C0_4B_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_477530 -> sub_425690: screen, zero time per stage.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_SET_RAIN_ORIGIN,
        symbol: "UserC0_4C_SetRainOrigin",
        parameters: USER_C0_4C_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4775B0: unknown screen fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_SET_RAIN_DIRECTION,
        symbol: "UserC0_4D_SetRainDirection",
        parameters: USER_C0_4D_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_477610: unknown screen fatal.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_SET_RAIN_LENGTH,
        symbol: "UserC0_4E_SetRainLength",
        parameters: USER_C0_4E_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_477670 -> sub_425780: screen, zero projection.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_SET_RAIN_STEP,
        symbol: "UserC0_4F_SetRainStep",
        parameters: USER_C0_4F_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4776F0 -> sub_492D80: frame rate 1..=1000.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_CREATE_SPLINE,
        symbol: "UserC0_C0_CreateSpline",
        parameters: NO_PARAMETERS,
        returns: "spline handle",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_477760 -> sub_495420: new spline id.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_RELEASE_SPLINE,
        symbol: "UserC0_C1_ReleaseSpline",
        parameters: USER_C0_C1_PARAMETERS,
        returns: "0 success, 1 missing",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_477780 -> sub_4954D0: 1 when unknown.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_CONFIGURE_SPLINE,
        symbol: "UserC0_C2_ConfigureSpline",
        parameters: USER_C0_C2_PARAMETERS,
        returns: "0 success, 1 missing, 2 count, 3 duration",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4777B0 -> sub_495550: [handle, count, points, duration]; count < 2 (2), duration < 2 (3) unsigned, then handle (1).",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_SAMPLE_SPLINE,
        symbol: "UserC0_C3_SampleSpline",
        parameters: USER_C0_C3_PARAMETERS,
        returns: "0 success, 1 missing, 4 range, -1 failure",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_477800 -> sub_4955D0: handle 1, time beyond duration 4, -1 on sampling failure.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::USER2_LOAD_BWEF_TABLE,
        symbol: "UserC0_F0_LoadBwefTable",
        parameters: USER_C0_F0_PARAMETERS,
        returns: "target BWEF status",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_477840 -> sub_4066C0: BWEF table load.",
    },
    NativeOpcodeSpec {
        opcode: opcodes::GRAPH_SET_PERSISTENT_TEXT_STYLE,
        symbol: "Graph92_97_SetPersistentTextStyleFields",
        parameters: GRAPH_PERSISTENT_TEXT_STYLE_PARAMETERS,
        returns: "void",
        scheduling: NativeSchedulingEffect::Continue,
        notes: "sub_4870E0 -> sub_463820: font number resolved through sub_469110, six fields.",
    },
];

// Recovery classifications are source-level audit data. They are kept beside
// the typed wrappers so generated documentation and runtime code cannot drift.
// External decompiler names are never sufficient for `TargetConfirmed`.
const TARGET_CONFIRMED_OPCODES: &[NativeOpcode] = &[
    opcodes::USER2_CREATE_PARTICLE_SCREEN,
    opcodes::USER2_RELEASE_PARTICLE_SCREEN,
    opcodes::USER2_SET_PARTICLE_SCREEN_ENABLED,
    opcodes::USER2_CONFIGURE_PARTICLE_SCREEN_DISPLAY,
    opcodes::USER2_CONFIGURE_PARTICLE_FRAME_TABLES,
    opcodes::USER2_COMMIT_PARTICLE_SCREEN,
    opcodes::USER2_SET_PARTICLE_AUTO_UPDATE,
    opcodes::USER2_SET_PARTICLE_CAPACITY,
    opcodes::USER2_CONFIGURE_PARTICLE_EMITTER_TRANSFORM,
    opcodes::USER2_SET_PARTICLE_EMISSION_PERCENT,
    opcodes::USER2_ADVANCE_PARTICLE_SCREEN,
    opcodes::USER2_RESET_PARTICLE_SCREEN,
    opcodes::USER2_CONFIGURE_PARTICLE_EMITTER,
    opcodes::USER2_CONFIGURE_PARTICLE_ANIMATION_BANK,
    opcodes::USER2_LOAD_PARTICLE_ANIMATION_FRAMES,
    opcodes::USER2_COMMIT_PARTICLE_ANIMATION_FRAMES,
    opcodes::USER2_SET_PARTICLE_INTERPOLATION_MODE,
    opcodes::USER2_SET_PARTICLE_STYLE0,
    opcodes::USER2_DEFINE_PARTICLE_STYLE0,
    opcodes::USER2_CONFIGURE_PARTICLE_STYLE0,
    opcodes::USER2_SET_PARTICLE_STYLE1,
    opcodes::USER2_CONFIGURE_PARTICLE_STYLE1,
    opcodes::USER2_DEFINE_PARTICLE_STYLE1,
    opcodes::USER2_CONFIGURE_PARTICLE_ADVANCED_STYLE,
    opcodes::USER2_CREATE_RAIN_SCREEN,
    opcodes::USER2_RELEASE_RAIN_SCREEN,
    opcodes::USER2_INITIALIZE_RAIN_SCREEN,
    opcodes::USER2_SET_RAIN_TEXTURE,
    opcodes::USER2_SET_RAIN_ENABLED,
    opcodes::USER2_CONFIGURE_RAIN_DISPLAY,
    opcodes::USER2_SET_RAIN_VOLUME_BOUNDS,
    opcodes::USER2_SET_RAIN_DROP_WIDTH,
    opcodes::USER2_SET_RAIN_DROP_HEIGHT,
    opcodes::USER2_SET_RAIN_COLOR,
    opcodes::USER2_SET_RAIN_DENSITY,
    opcodes::USER2_SET_RAIN_SPEED,
    opcodes::USER2_SET_RAIN_ORIGIN,
    opcodes::USER2_SET_RAIN_DIRECTION,
    opcodes::USER2_SET_RAIN_LENGTH,
    opcodes::USER2_SET_RAIN_STEP,
    opcodes::USER2_CREATE_SPLINE,
    opcodes::USER2_RELEASE_SPLINE,
    opcodes::USER2_CONFIGURE_SPLINE,
    opcodes::USER2_SAMPLE_SPLINE,
    opcodes::USER2_LOAD_BWEF_TABLE,
    opcodes::SYS81_SET_CLOCK_JUMP_THRESHOLD,
    opcodes::SYS81_COPY_COORDINATE_SLOT,
    opcodes::SYS81_GET_USER_NAME,
    opcodes::SYS81_GET_COMPUTER_NAME,
    opcodes::SYS81_GET_CPU_BRAND,
    opcodes::SYS81_GET_CPU_DISPLAY_INFO,
    opcodes::SYS81_GET_OS_VERSION,
    opcodes::SYS81_GET_PHYSICAL_MEMORY_MB,
    opcodes::SYS81_GET_ADJUSTED_DESKTOP_DIMENSIONS,
    opcodes::SYS81_IS_MAIN_WINDOW_MINIMIZED,
    opcodes::SYS81_SWAP_INPUT_BINDING,
    opcodes::SYS81_COPY_KEYBOARD_STATE,
    opcodes::SYS81_SET_KEYBOARD_POLLING_OVERRIDE,
    opcodes::SYS81_CONFIGURE_POINTER_HISTORY,
    opcodes::SYS81_COPY_POINTER_HISTORY,
    opcodes::SYS81_REGISTER_TOUCH_INPUT,
    opcodes::SYS81_COPY_TOUCH_RECORDS,
    opcodes::SYS81_SET_CONTROLLER_WAKE_ENTRY,
    opcodes::SYS81_QUERY_CONTROLLER_STATE,
    opcodes::SYS81_INJECT_MOUSE_CLICK,
    opcodes::SYS81_OPEN_RESOURCE_STREAM,
    opcodes::SYS81_QUEUE_RESOURCE_CLOSE,
    opcodes::SYS81_QUEUE_RESOURCE_READ,
    opcodes::SYS81_QUEUE_RESOURCE_SEEK,
    opcodes::SYS81_GET_FILE_TIMES,
    opcodes::SYS81_SET_FILE_TIMES,
    opcodes::SYS81_TEST_PATH_WRITABLE,
    opcodes::SYS81_READ_RESOURCE_BINARY,
    opcodes::SYS81_INTERNET_READ,
    opcodes::SYS81_DIRECT_FILE_READ,
    opcodes::SYS81_RESOURCE_SIZE,
    opcodes::SYS81_ENUMERATE_DRIVE_TYPES,
    opcodes::SYS81_GET_DISK_FREE_MB,
    opcodes::SYS81_SHOW_RESOURCE_FILE_DIALOG,
    opcodes::SYS81_ENUMERATE_RESOURCES,
    opcodes::SYS81_BROWSE_FOLDER,
    opcodes::SYS81_SHOW_RESOURCE_LIST,
    opcodes::SYS81_RESOURCE_EXISTS,
    opcodes::SYS81_GET_VOLUME_LABEL,
    opcodes::SYS81_QUERY_DEVICE_POWER_STATE,
    opcodes::SYS81_CREATE_CHILD_THREAD,
    opcodes::SYS81_CONFIGURE_SCREEN_MODE,
    opcodes::SYS81_QUERY_EFFECTIVE_DISPLAY_MODE,
    opcodes::SYS81_SET_MONITOR_ADAPTER_MODE,
    opcodes::SYS81_SET_CONFIG_INPUT_MODE,
    opcodes::SYS81_CONFIGURE_LOGICAL_SCREEN_SIZE,
    opcodes::SYS81_SET_WINDOW_POSITION_OVERRIDE,
    opcodes::SYS81_SWAP_PAUSE_ON_DEACTIVATE,
    opcodes::SYS81_SET_PRINT_SCREEN_HOTKEY_CAPTURE,
    opcodes::SYS81_SET_ERROR_CAPTURE_MODE,
    opcodes::SYS81_COPY_CAPTURED_ERROR,
    opcodes::SYS81_QUERY_PIXEL_SHADER_VERSION,
    opcodes::SYS81_UNCAUGHT_EXCEPTION_ACTIVE,
    opcodes::SYS81_SET_BICUBIC_SHADER,
    opcodes::SYS81_WIDE_STRING_DISTANCE,
    opcodes::SYS81_SHIFT_JIS_TO_UTF16,
    opcodes::SYS81_BLOB_TABLE_OPEN,
    opcodes::SYS81_BLOB_TABLE_CLOSE,
    opcodes::SYS81_BLOB_TABLE_STORE,
    opcodes::SYS81_BLOB_TABLE_REMOVE,
    opcodes::SYS81_BLOB_TABLE_FETCH,
    opcodes::SYS81_BLOB_TABLE_ENUMERATE,
    opcodes::SYS81_LAUNCH_PROCESS_WAIT,
    opcodes::SYS81_UPDATE_ROLLING_HASH,
    opcodes::SYS81_MD5,
    opcodes::SYS81_CREATE_NAMED_MUTEX,
    opcodes::SYS81_RELEASE_NAMED_MUTEX,
    opcodes::SYS81_RUN_INSTALLATION_PROCEDURE,
    opcodes::SYS81_VALIDATE_OR_CREATE_USER_PATH,
    opcodes::SYS_SRAND,
    opcodes::SYS_RAND,
    opcodes::SYS_RAND_MAX,
    opcodes::SYS_GET_ENGINE_TICK,
    opcodes::SYS_QUERY_PERFORMANCE_COUNTER_NS,
    opcodes::SYS_SET_PERFORMANCE_PROFILING,
    opcodes::SYS_READ_PERFORMANCE_METRIC,
    opcodes::SYS_READ_CURSOR_POINT,
    opcodes::SYS_QUERY_PRESENTATION_STATE,
    opcodes::SYS_COPY_GRAPHICS_CAPABILITIES,
    opcodes::SYS_QUERY_GRAPHICS_MEMORY_METRIC,
    opcodes::SYS_GET_LOCAL_TIME,
    opcodes::SYS_GET_PHYSICAL_MEMORY,
    opcodes::SYS_QUERY_WINDOW_MINIMIZE_LATCH,
    opcodes::SYS_QUERY_WINDOW_ACTIVE,
    opcodes::SYS_RESET_INPUT_CONFIGURATION,
    opcodes::SYS_QUERY_KEY_DOWN,
    opcodes::SYS_SUM_INPUT_DESCRIPTOR_STATE,
    opcodes::SYS_SET_MOUSE_BUTTON_MAPPING_MODE,
    opcodes::SYS_ALLOC,
    opcodes::SYS_FREE,
    opcodes::SYS_COUNT_FILES,
    opcodes::SYS_ENUMERATE_FILES,
    opcodes::SYS_ENUMERATE_DIRECTORIES,
    opcodes::SYS_MOVE_FILE,
    opcodes::SYS_CREATE_DIRECTORY,
    opcodes::SYS_REMOVE_DIRECTORY,
    opcodes::SYS_DIRECTORY_EXISTS,
    opcodes::SYS_SPLIT_PATH,
    opcodes::SYS_GET_FILE_ATTRIBUTES,
    opcodes::SYS_SET_FILE_ATTRIBUTES,
    opcodes::SYS_COPY_FILE,
    opcodes::SYS_READ_FILE_BYTES,
    opcodes::SYS_READ_FILE_RANGE,
    opcodes::SYS_WRITE_FILE_BYTES,
    opcodes::SYS_SET_ADDITIONAL_RESOURCE_SEARCH,
    opcodes::SYS_PREPEND_RESOURCE_SEARCH_PATH,
    opcodes::SYS_REGISTER_COMPOSITE_ARCHIVE,
    opcodes::SYS_RTC_NOOP,
    opcodes::SYS_CURRENT_THREAD_ID,
    opcodes::SYS_THREAD_EXISTS,
    opcodes::SYS_DELETE_FILE,
    opcodes::SYS_FILE_EXISTS,
    opcodes::SYS_FILE_SIZE,
    opcodes::SYS_SET_VALIDATED_FILE_ROOT,
    opcodes::SYS_GET_SPECIAL_FOLDER,
    opcodes::SYS_OPEN_FILE_DIALOG,
    opcodes::SYS_REQUIRE_RESOURCE_FILE,
    opcodes::SYS_GET_CONFIGURED_ROOT,
    opcodes::SYS_SET_PRIMARY_ROOT,
    opcodes::SYS_CONFIGURE_REMOVABLE_ARCHIVE,
    opcodes::SYS_LOAD_PROGRAM_MODULE,
    opcodes::SYS_FREE_LAST_PROGRAM_MODULE,
    opcodes::SYS_LOAD_PROGRAM_THREAD,
    opcodes::SYS_INPUT_MESSAGE_SERIAL,
    opcodes::SYS_SET_INPUT_MASTER_GATE,
    opcodes::SYS_SET_INPUT_LATCHED_STATE,
    opcodes::SYS_SAMPLE_CONFIGURED_INPUT,
    opcodes::SYS_QUERY_CONFIGURED_INPUT_GATE,
    opcodes::SYS_REGISTER_INPUT_SCOPE,
    opcodes::SYS_QUERY_AND_UNREGISTER_INPUT_SCOPE,
    opcodes::SYS_QUERY_INPUT_EVENT_BITS,
    opcodes::SYS_REGISTER_INPUT_CLASS_DESCRIPTORS,
    opcodes::SYS_QUERY_INPUT_CLASS_LEVEL,
    opcodes::SYS_QUERY_SCOPED_INPUT_EVENT,
    opcodes::SYS_CONFIGURE_CURSOR_MOTION,
    opcodes::SYS_SET_MESSAGE_AUXILIARY_INPUT_MASK,
    opcodes::SYS_ENQUEUE_MESSAGE,
    opcodes::SYS_DEQUEUE_MESSAGE,
    opcodes::SYS_ENQUEUE_MESSAGE_ARRAY,
    opcodes::SYS_DEQUEUE_MESSAGE_ARRAY,
    opcodes::SYS_INVOKE_THREAD_CALLBACK,
    opcodes::SYS_SET_SYSTEM_WAIT_STATE,
    opcodes::SYS_WAIT_WINDOW_MESSAGE,
    opcodes::SYS_SET_THREAD_TIMER,
    opcodes::SYS_SET_AND_QUERY_THREAD_TIMER,
    opcodes::SYS_WAIT_THREAD_TIMER,
    opcodes::SYS_WAIT_TIMING_EX,
    opcodes::SYS_SWITCH_PROGRAM,
    opcodes::SYS_YIELD,
    opcodes::SYS_SET_MAIN_LOOP_WAIT_OVERRIDE,
    opcodes::SYS_ARM_NEXT_BINARY_ASYNC,
    opcodes::SYS_SET_EXCLUSIVE_THREAD,
    opcodes::SYS_CONFIGURE_DISPLAY_MODE,
    opcodes::SYS_QUERY_FULLSCREEN,
    opcodes::SYS_CONFIGURE_FULLSCREEN_HOTKEYS,
    opcodes::SYS_SET_ASPECT_PRESERVING_SCALING,
    opcodes::SYS_SET_WINDOW_VISIBLE,
    opcodes::SYS_MINIMIZE_WINDOW,
    opcodes::SYS_SET_WINDOW_TITLE,
    opcodes::SYS_SET_CURSOR_INDEX,
    opcodes::SYS_SET_NATIVE_CLOSE_MODE,
    opcodes::SYS_REQUEST_WINDOW_CLOSE,
    opcodes::SYS_TERMINATE_INTERPRETER,
    opcodes::SYS_SELECT_BOOTSTRAP,
    opcodes::SYS_SET_FILE_DROP_ENABLED,
    opcodes::SYS_QUERY_DROPPED_FILE,
    opcodes::SYS_SET_RASTER_WAIT_ENABLED,
    opcodes::SYS_QUERY_DISPLAY_ASPECT_MISMATCH,
    opcodes::SYS_ALLOCATE_GLOBAL_CONFIG,
    opcodes::SYS_CLEAR_GLOBAL_CONFIG,
    opcodes::SYS_SET_SAVE_DATA_INTEGRITY,
    opcodes::SYS_SAVE_CONFIG_SLOT,
    opcodes::SYS_LOAD_CONFIG_SLOT,
    opcodes::SYS_READ_CONFIG_SLOT_HEADER,
    opcodes::SYS_VALIDATE_CONFIG_SLOT,
    opcodes::SYS_LOAD_GLOBAL_USER_DATA,
    opcodes::SYS_SAVE_GLOBAL_USER_DATA,
    opcodes::SYS_WRITE_GLOBAL_DATA_BLOCK,
    opcodes::SYS_READ_GLOBAL_DATA_BLOCK,
    opcodes::SYS_RESET_STRUCTURED_HISTORY,
    opcodes::SYS_STRUCTURED_HISTORY_COUNT,
    opcodes::SYS_APPEND_STRUCTURED_HISTORY_FIELDS,
    opcodes::SYS_READ_STRUCTURED_HISTORY,
    opcodes::SYS_APPEND_STRUCTURED_HISTORY_RECORD,
    opcodes::SYS_READ_STRUCTURED_HISTORY_EXTENDED,
    opcodes::SYS_INDEXED_RECORD_OPEN,
    opcodes::SYS_INDEXED_RECORD_CLOSE,
    opcodes::SYS_INDEXED_RECORD_COUNT,
    opcodes::SYS_INDEXED_RECORD_PUSH,
    opcodes::SYS_INDEXED_RECORD_LOAD,
    opcodes::SYS_INDEXED_RECORD_REMOVE,
    opcodes::SYS_POLL_QUEUED_EVENT,
    opcodes::SYS_POST_QUEUED_EVENT,
    opcodes::SYS_SET_REGISTERED_OBJECT_STATE,
    opcodes::SYS_GET_REGISTERED_OBJECT_STATE,
    opcodes::SYS_QUEUE_REGISTERED_OBJECT_MESSAGE,
    opcodes::SYS_SET_SYSTEM_MODE_FLAG,
    opcodes::SYS_CREATE_EXCLUSION_SECTION,
    opcodes::SYS_DELETE_EXCLUSION_SECTION,
    opcodes::SYS_WAIT_EXCLUSION_SECTION,
    opcodes::SYS_LEAVE_EXCLUSION_SECTION,
    opcodes::SYS_QUERY_EXCLUSION_SECTION,
    opcodes::SYS_INTERN_RESOURCE_NAME,
    opcodes::SYS_RESOURCE_NAME_EXISTS,
    opcodes::SYS_CREATE_OR_RESIZE_READ_FLAG_TABLE,
    opcodes::SYS_SET_READ_FLAG_BIT,
    opcodes::SYS_SET_READ_FLAG_RANGE,
    opcodes::SYS_QUERY_READ_FLAG_BIT,
    opcodes::SYS_ENCODE_DATA,
    opcodes::SYS_DECODE_SDC_BUFFER,
    opcodes::SYS_ENCODE_STRUCT_ARRAY,
    opcodes::SYS_DECODE_STRUCT_ARRAY,
    opcodes::SYS_DECODE_DATA,
    opcodes::SYS_RECORD_TABLE_OPEN,
    opcodes::SYS_RECORD_TABLE_CLOSE,
    opcodes::SYS_RECORD_TABLE_INSERT,
    opcodes::SYS_RECORD_TABLE_REMOVE,
    opcodes::SYS_RECORD_TABLE_FETCH,
    opcodes::SYS_CLEAR_STRING_NAMESPACES,
    opcodes::SYS_STRING_NAMESPACE_COUNT,
    opcodes::SYS_REPLACE_STRING_NAMESPACE,
    opcodes::SYS_SERIALIZE_STRING_NAMESPACE,
    opcodes::SYS_INTERN_STRING_NAMESPACE,
    opcodes::SYS_STRING_NAMESPACE_ENTRY_LENGTH,
    opcodes::SYS_LAUNCH_PROCESS_WAIT,
    opcodes::SYS_RESTART_WITH_COMMAND,
    opcodes::SYS_LAUNCH_PROCESS_WAIT_UNINSTALLER,
    opcodes::SYS_SHELL_OPEN,
    opcodes::SYS_WRITE_GAME_ID,
    opcodes::SYS_HASH_FILE,
    opcodes::SYS_SET_UNINSTALLER_PRODUCT,
    opcodes::SYS_SHOW_INPUT_DIALOG,
    opcodes::SYS_SHOW_INSTALLER_DIALOG,
    opcodes::SYS_RUN_INSTALLER_WORKFLOW,
    opcodes::SYS_RUN_SHORTCUT_INSTALLER_WORKFLOW,
    opcodes::SYS_REMOVE_UNINSTALL_LISTED_FILES,
    opcodes::SYS_APPEND_UNINSTALL_LIST_ENTRIES,
    opcodes::SYS_REMOVE_INSTALLER_SHORTCUTS,
    opcodes::SYS_CREATE_SHORTCUT,
    opcodes::SYS_READ_INSTALLED_FOLDER,
    opcodes::SYS_DELETE_INSTALLED_REGISTRY_KEY,
    opcodes::SYS_READ_WINDOWS_PATH_FILE,
    opcodes::SYS_WRITE_WINDOWS_DIRECTORY,
    opcodes::SYS_REGISTER_FILE_ASSOCIATION,
    opcodes::SYS_QUERY_LAUNCHER_MODE,
    opcodes::SYS_INSTALLER_FEATURE_AVAILABLE,
    opcodes::SYS_COPY_INDEXED_NAMESPACE_RECORD,
    opcodes::GRAPH90_REQUEST_REDRAW,
    opcodes::GRAPH90_SET_SCHEDULER_GATE,
    opcodes::GRAPH90_SET_FRAME_RATE,
    opcodes::GRAPH90_INITIALIZE_BITMAP_MEMORY_MANAGER,
    opcodes::GRAPH90_CREATE_WORK_BITMAP,
    opcodes::GRAPH90_CREATE_PRIORITIZED_WORK_BITMAP,
    opcodes::GRAPH90_SET_CENTER,
    opcodes::GRAPH90_SET_SYNC_LOAD_HOLD,
    opcodes::GRAPH90_SET_DISPLAY_ENABLED,
    opcodes::GRAPH90_SET_DEFAULT_PRIORITY,
    opcodes::GRAPH90_SET_OBJECT_UPDATE_REDRAW_POLICY,
    opcodes::GRAPH90_SET_CURRENT_BITMAP,
    opcodes::GRAPH90_SET_DISPLAY_OPTIONS,
    opcodes::GRAPH90_SET_RASTER_FORMAT_MODE,
    opcodes::GRAPH90_REGISTER_FONT,
    opcodes::GRAPH90_SET_BITMAP_UNBLEND_COLOR,
    opcodes::GRAPH90_LOAD_BITMAP,
    opcodes::GRAPH90_CREATE_BITMAP,
    opcodes::GRAPH90_RELEASE_BITMAP,
    opcodes::GRAPH90_FILL_BITMAP,
    opcodes::GRAPH90_CREATE_BITMAP_FROM_PIXELS,
    opcodes::GRAPH90_COPY_BITMAP_PIXELS,
    opcodes::GRAPH90_QUERY_BITMAP_INFO,
    opcodes::GRAPH90_VALIDATE_BITMAP_FORMAT,
    opcodes::GRAPH90_BLIT_BITMAP,
    opcodes::GRAPH90_SYNTHESIZE_BITMAP,
    opcodes::GRAPH90_COMPOSITE_BITMAPS,
    opcodes::GRAPH90_COPY_BITMAP,
    opcodes::GRAPH90_SCALE_BITMAP_REGION,
    opcodes::GRAPH90_TRANSFORM_BITMAP,
    opcodes::GRAPH90_BLIT_BITMAP_REGION,
    opcodes::GRAPH90_CREATE_BITMAP_REGION,
    opcodes::GRAPH90_START_OBJECT_CONTROL,
    opcodes::GRAPH90_START_OBJECT_CONTROL_EX,
    opcodes::GRAPH90_START_NODE_CONTROL,
    opcodes::GRAPH90_START_NODE_CONTROL_EX,
    opcodes::GRAPH90_START_SPECIAL_OBJECT_CONTROL,
    opcodes::GRAPH90_START_OBJECT_MOTION_CONTROL,
    opcodes::GRAPH90_START_SPLINE_OBJECT_CONTROL,
    opcodes::GRAPH90_START_SHAKE_OBJECT_CONTROL,
    opcodes::GRAPH90_SET_OBJECT_DRAW_ENABLED,
    opcodes::GRAPH90_SET_OBJECT_ENABLED,
    opcodes::GRAPH90_SET_OBJECT_POSITION,
    opcodes::GRAPH90_SET_OBJECT_MASK_ALPHA,
    opcodes::GRAPH90_SET_OBJECT_FIXED_PARAMETER,
    opcodes::GRAPH90_SET_OBJECT_SECONDARY_OFFSET,
    opcodes::GRAPH90_SET_OBJECT_PRIMARY_OFFSET,
    opcodes::GRAPH90_SET_OBJECT_PROPERTY,
    opcodes::GRAPH90_SET_OBJECT_ALPHA_MULTIPLIER,
    opcodes::GRAPH90_SET_OBJECT_PRIORITY,
    opcodes::GRAPH90_SET_OBJECT_HIT_MASK_BITMAP,
    opcodes::GRAPH90_HIT_TEST_OBJECT_AT_POINTER,
    opcodes::GRAPH90_INVOKE_UNSUPPORTED_OBJECT_EXTENSION,
    opcodes::GRAPH_SET_OBJECT_ALPHA,
    opcodes::GRAPH90_SET_CURRENT_OBJECT_BITMAP,
    opcodes::GRAPH90_CONFIGURE_CURRENT_OBJECT_DUAL_BITMAP,
    opcodes::GRAPH90_CONFIGURE_CURRENT_OBJECT_QUAD_BITMAP,
    opcodes::GRAPH90_CONFIGURE_CURRENT_OBJECT_SPRITE_MASK,
    opcodes::GRAPH90_CONFIGURE_CURRENT_OBJECT_FRAME_TABLE,
    opcodes::GRAPH90_CONFIGURE_CURRENT_OBJECT_RESOURCE_TRIPLET,
    opcodes::GRAPH90_CONFIGURE_CURRENT_OBJECT_BITMAP_MODE,
    opcodes::GRAPH90_CONFIGURE_CURRENT_OBJECT_VM_EFFECT,
    opcodes::GRAPH90_CONFIGURE_CURRENT_OBJECT_BITMAP_SIZE_POSITION,
    opcodes::GRAPH90_CONFIGURE_CURRENT_OBJECT_BITMAP_SIZE,
    opcodes::GRAPH90_CONFIGURE_CURRENT_OBJECT_BLIT_SOURCES,
    opcodes::GRAPH90_SET_CURRENT_OBJECT_RENDER_CONTROLS,
    opcodes::GRAPH90_GET_CURRENT_OBJECT_MODE,
    opcodes::GRAPH90_CREATE_SPRITE_OBJECT,
    opcodes::GRAPH90_RELEASE_SPRITE_OBJECT,
    opcodes::GRAPH90_REFRESH_SPRITE_OBJECT,
    opcodes::GRAPH90_SET_SPRITE_ENABLED,
    opcodes::GRAPH90_SET_SPRITE_AUX_BITMAP,
    opcodes::GRAPH90_CONFIGURE_SPRITE_SINGLE_BITMAP,
    opcodes::GRAPH90_REPLACE_SPRITE_BITMAP,
    opcodes::GRAPH90_CONFIGURE_SPRITE_DUAL_BITMAP,
    opcodes::GRAPH90_CONFIGURE_SPRITE_SCALED_BITMAP,
    opcodes::GRAPH90_CONFIGURE_SPRITE_MASKED_BITMAP,
    opcodes::GRAPH90_CONFIGURE_SPRITE_VM_EFFECT,
    opcodes::GRAPH90_CONFIGURE_SPRITE_TRANSFORM_MODE5,
    opcodes::GRAPH90_CONFIGURE_SPRITE_TRANSFORM_MODE6,
    opcodes::GRAPH90_CREATE_FILTER_OBJECT,
    opcodes::GRAPH90_RELEASE_FILTER_OBJECT,
    opcodes::GRAPH90_SET_FILTER_ENABLED,
    opcodes::GRAPH90_CONFIGURE_FILTER,
    opcodes::GRAPH90_CONFIGURE_FILTER_WITH_MASK,
    opcodes::GRAPH90_CREATE_MAP_OBJECT,
    opcodes::GRAPH90_RELEASE_MAP_OBJECT,
    opcodes::GRAPH90_SET_MAP_ENABLED,
    opcodes::GRAPH90_CONFIGURE_MAP_OBJECT,
    opcodes::GRAPH90_INITIALIZE_MAP_GRID,
    opcodes::GRAPH90_UPLOAD_MAP_TILE_DATA,
    opcodes::GRAPH90_SET_MAP_VIEWPORT,
    opcodes::GRAPH90_REPLACE_MAP_TILE_ID,
    opcodes::GRAPH90_CREATE_WINDOW_OBJECT,
    opcodes::GRAPH90_RELEASE_WINDOW_OBJECT,
    opcodes::GRAPH90_SET_WINDOW_COMPOSITION_ORDER,
    opcodes::GRAPH90_SET_WINDOW_DRAW_ENABLED,
    opcodes::GRAPH90_CONFIGURE_WINDOW_OBJECT,
    opcodes::GRAPH90_SET_WINDOW_ISOLATED_COMPOSITION,
    opcodes::GRAPH90_SET_WINDOW_VALID_REGION,
    opcodes::GRAPH90_GET_WINDOW_VALID_REGION,
    opcodes::GRAPH90_START_WINDOW_MESSAGE_PROCEDURE,
    opcodes::GRAPH90_CONFIGURE_MESSAGE_CARET_FRAMES,
    opcodes::GRAPH90_SET_MESSAGE_CARET_FRAME_DELAY,
    opcodes::GRAPH90_SET_MESSAGE_CARET_POSITION,
    opcodes::GRAPH90_SET_TEXT_SHADOW_ENABLED,
    opcodes::GRAPH90_SET_TEXT_SHADOW_PARAMETERS,
    opcodes::GRAPH90_REGISTER_FULLWIDTH_GLYPH_STRIP,
    opcodes::GRAPH90_START_ITEM_SELECTION,
    opcodes::GRAPH90_DRAW_ITEM_SELECTION_GRID,
    opcodes::GRAPH90_START_ITEM_SELECTION_EX,
    opcodes::GRAPH90_START_ITEM_SELECTION_EX_BLINK,
    opcodes::GRAPH90_SET_ITEM_SELECTION_HIGHLIGHT_STYLES,
    opcodes::GRAPH90_SET_ITEM_SELECTION_INPUT_MASK,
    opcodes::GRAPH90_SET_ITEM_SELECTION_NAVIGATION_PARAMETERS,
    opcodes::GRAPH90_SET_ITEM_SELECTION_COLUMN_LAYOUT,
    opcodes::GRAPH90_SET_INTERACTIVE_PROCEDURE_POLL_GATE,
    opcodes::GRAPH90_START_ICON_SELECTION,
    opcodes::GRAPH90_START_ICON_SELECTION_EX,
    opcodes::GRAPH90_DRAW_ICON_BATCH,
    opcodes::GRAPH90_DRAW_EXTENDED_ICON_BATCH,
    opcodes::GRAPH90_APPLY_ICON_INPUT_LAYOUT,
    opcodes::GRAPH90_APPLY_ICON_INPUT_LAYOUT_EX,
    opcodes::GRAPH90_CREATE_ICON_INPUT_PROCESSOR,
    opcodes::GRAPH90_RELEASE_ICON_INPUT_PROCESSOR,
    opcodes::GRAPH90_CONFIGURE_ICON_INPUT_PROCESSOR,
    opcodes::GRAPH90_GET_ICON_INPUT_STATE,
    opcodes::GRAPH90_GET_ICON_INPUT_CURRENT_GROUP,
    opcodes::GRAPH90_GET_ICON_INPUT_SELECTIONS,
    opcodes::GRAPH90_POP_ICON_INPUT_EVENT,
    opcodes::GRAPH90_LOAD_BG_BITMAP_RESOURCE,
    opcodes::GRAPH90_FLIP_BITMAP,
    opcodes::GRAPH90_DOWNSAMPLE_BITMAP_HALF,
    opcodes::GRAPH90_IMPORT_EXTERNAL_IMAGE,
    opcodes::GRAPH90_SAVE_BITMAP_TO_IMAGE_FILE,
    opcodes::GRAPH90_REGISTER_BG_RESOURCE_DATA,
    opcodes::GRAPH90_LOAD_CACHED_BG_BITMAP,
    opcodes::GRAPH90_TRANSFORM_BITMAP_GENERAL,
    opcodes::GRAPH90_SCALE_BITMAP_ASPECT_FIT,
    opcodes::GRAPH90_REGISTER_TONE_CURVE,
    opcodes::GRAPH90_APPLY_TONE_CURVE_EFFECT,
    opcodes::GRAPH90_ENCODE_BITMAP_TO_BUFFER,
    opcodes::GRAPH90_CREATE_KNOB_OBJECT,
    opcodes::GRAPH90_RELEASE_KNOB_OBJECT,
    opcodes::GRAPH90_SET_KNOB_ENABLED,
    opcodes::GRAPH90_SET_KNOB_BASE_POSITION,
    opcodes::GRAPH90_SET_KNOB_POSITION,
    opcodes::GRAPH90_GET_KNOB_POSITION,
    opcodes::GRAPH90_SET_KNOB_MOVEMENT_PRECISION,
    opcodes::GRAPH90_SET_KNOB_MOVEMENT_RANGE,
    opcodes::GRAPH90_TAKE_KNOB_VERTICAL_EVENT,
    opcodes::GRAPH90_TAKE_CHANGED_KNOB_HANDLE,
    opcodes::GRAPH90_SET_KNOB_RELATIVE_MODE,
    opcodes::GRAPH90_SWAP_KNOB_INPUT_MODE,
    opcodes::GRAPH90_WATCH_KNOB_OBJECT,
    opcodes::GRAPH90_UNWATCH_KNOB_OBJECT,
    opcodes::GRAPH90_CREATE_GROUP_OBJECT,
    opcodes::GRAPH90_RELEASE_GROUP_OBJECT,
    opcodes::GRAPH90_SET_GROUP_ENABLED,
    opcodes::GRAPH90_CONFIGURE_GROUP_OBJECT,
    opcodes::GRAPH90_ADD_OBJECT_TO_GROUP,
    opcodes::GRAPH90_REMOVE_OBJECT_FROM_GROUP,
    opcodes::GRAPH90_OPEN_DIRECTSHOW_MOVIE,
    opcodes::GRAPH90_CLOSE_DIRECTSHOW_MOVIE,
    opcodes::GRAPH90_IS_DIRECTSHOW_MOVIE_PLAYING,
    opcodes::GRAPH90_SET_DIRECTSHOW_MOVIE_VOLUME,
    opcodes::GRAPH90_LOAD_BURIKO_MOVIE_RESOURCE,
    opcodes::GRAPH90_RELEASE_BURIKO_MOVIE_RESOURCE,
    opcodes::GRAPH90_DECODE_BURIKO_MOVIE_FRAME,
    opcodes::GRAPH90_ATTACH_BURIKO_MOVIE_RESOURCE,
    opcodes::GRAPH90_CLEAR_SPRITE_TARGETS,
    opcodes::GRAPH90_REGISTER_SPRITE_TARGET,
    opcodes::GRAPH90_UNREGISTER_SPRITE_TARGET,
    opcodes::GRAPH90_HIT_TEST_SPRITE_TARGETS,
    opcodes::GRAPH90_GET_SPRITE_TARGET_STATE,
    opcodes::GRAPH_SET_MESSAGE_START_DELAY,
    opcodes::GRAPH_RENDER_OBJECT_TO_BITMAP,
    opcodes::GRAPH_BIND_BITMAP_TO_SURFACE,
    opcodes::GRAPH92_CONFIGURE_COMPACT_WAVE_TABLE,
    opcodes::GRAPH92_CONFIGURE_WAVE_TABLE,
    opcodes::GRAPH92_GENERATE_RADIAL_VECTOR_MAP,
    opcodes::GRAPH92_GENERATE_AXIS_VECTOR_MAP,
    opcodes::GRAPH92_SET_BITMAP_AUXILIARY_PAIR,
    opcodes::GRAPH92_REPLACE_BITMAP_COLOR,
    opcodes::GRAPH92_PRELOAD_BITMAP_RESOURCE,
    opcodes::GRAPH92_CANCEL_PENDING_BITMAP_PRELOADS,
    opcodes::GRAPH92_GET_BITMAP_AUXILIARY_PAIR,
    opcodes::GRAPH92_READ_BITMAP_PIXEL_VALUE,
    opcodes::GRAPH_CONVERT_BITMAP_TO_ALPHA_DESCRIPTOR,
    opcodes::GRAPH92_INVERT_ALPHA_BITMAP,
    opcodes::GRAPH92_COMPOSE_BITMAP_ALPHA_AT_OFFSET,
    opcodes::GRAPH92_DRAW_BITMAP_TEXT_MEASURE,
    opcodes::GRAPH92_DRAW_WRAPPED_BITMAP_TEXT,
    opcodes::GRAPH92_DRAW_BITMAP_TEXT,
    opcodes::GRAPH92_LOAD_EXTERNAL_BMP,
    opcodes::GRAPH_SET_MESSAGE_INPUT_SCOPE,
    opcodes::GRAPH_SET_MESSAGE_INPUT_FILTER,
    opcodes::GRAPH_SET_GLYPH_REVEAL_DELAY,
    opcodes::GRAPH_SET_TEXT_REVEAL_ANIMATION,
    opcodes::GRAPH_SET_TEXT_SETTLE_ANIMATION,
    opcodes::GRAPH_SET_TEXT_AUTO_ADVANCE,
    opcodes::GRAPH_SET_MESSAGE_INPUT_FORCES_COMPLETION,
    opcodes::GRAPH92_SET_TEXT_OBJECT_VALUE,
    opcodes::GRAPH92_COMPOSITE_RESOURCE_INTO_TEXT_OBJECT,
    opcodes::GRAPH92_SET_TEXT_OBJECT_AUXILIARY_VALUE,
    opcodes::GRAPH92_SET_TEXT_OBJECT_UPDATE_FLAG,
    opcodes::GRAPH92_APPLY_EFFECT_RESOURCE,
    opcodes::GRAPH92_RESET_TEXT_OBJECT,
    opcodes::GRAPH92_START_STYLED_MESSAGE,
    opcodes::GRAPH92_DRAW_FORMATTED_TEXT,
    opcodes::GRAPH92_CONFIGURE_TEXT_BITMAP_SLOT,
    opcodes::GRAPH92_GET_TEXT_OUTPUT_PAIR,
    opcodes::GRAPH92_RENDER_TEXT,
    opcodes::GRAPH92_CONFIGURE_FONT_OVERRIDE,
    opcodes::GRAPH92_DRAIN_TEXT_FRAGMENT_RECORDS,
    opcodes::GRAPH92_SET_TEXT_RENDER_OVERRIDE,
    opcodes::GRAPH92_OPEN_GLOBAL_DIRECTSHOW_MOVIE,
    opcodes::GRAPH92_LOAD_BURIKO_MOVIE_RESOURCE,
    opcodes::GRAPH92_OPEN_BITMAP_DIRECTSHOW_MOVIE,
    opcodes::GRAPH92_SEEK_BITMAP_DIRECTSHOW_MOVIE,
    opcodes::GRAPH92_GET_BITMAP_DIRECTSHOW_MOVIE_POSITION,
    opcodes::GRAPH92_SET_BITMAP_DIRECTSHOW_MOVIE_VOLUME,
    opcodes::GRAPH_SET_PERSISTENT_TEXT_STYLE,
    opcodes::GRAPH91_CACHE_BINARY_RESOURCE,
    opcodes::GRAPH91_SET_GLOBAL_DISPLAY_OFFSET,
    opcodes::GRAPH91_SET_SCRIPT_BITMAP_CONTEXT_BINDING_ENABLED,
    opcodes::GRAPH91_SET_GLYPH_COVERAGE_MODE,
    opcodes::GRAPH91_SET_FONT_PITCH_DETECTION_ENABLED,
    opcodes::GRAPH91_REGISTER_NAMED_FONT_TRANSFORM,
    opcodes::GRAPH91_CONFIGURE_NATIVE_FONT,
    opcodes::GRAPH91_GENERATE_AFFINE_DISPLACEMENT_MAP,
    opcodes::GRAPH91_GENERATE_RANDOM_DISPLACEMENT_MAP,
    opcodes::GRAPH91_GENERATE_RIPPLE_DISPLACEMENT_MAP,
    opcodes::GRAPH91_GENERATE_PERSPECTIVE_BEND_MAP,
    opcodes::GRAPH91_GENERATE_CURVATURE_DISPLACEMENT_MAP,
    opcodes::GRAPH91_GENERATE_RADIAL_LENS_DISPLACEMENT_MAP,
    opcodes::GRAPH91_GENERATE_SINE_DISPLACEMENT_MAP,
    opcodes::GRAPH91_GENERATE_RADIAL_WARP_DISPLACEMENT_MAP,
    opcodes::GRAPH91_COMPOSITE_BITMAP_RECT_ALPHA,
    opcodes::GRAPH91_COMPOSITE_BITMAP_RECT_CONVERTED,
    opcodes::GRAPH91_REPLACE_BITMAP_RGB_PRESERVE_ALPHA,
    opcodes::GRAPH91_CONCENTRATE_BITMAP,
    opcodes::GRAPH91_SCALE_TRUE_COLOR_BITMAP,
    opcodes::GRAPH91_PROCESS_BITMAP,
    opcodes::GRAPH91_APPLY_GRAYSCALE_MASK,
    opcodes::GRAPH91_CLONE_BITMAP,
    opcodes::GRAPH91_SET_OBJECT_SUPPRESSED,
    opcodes::GRAPH91_SET_OBJECT_FIXED_POSITION,
    opcodes::GRAPH91_SET_OBJECT_SECONDARY_VECTOR,
    opcodes::GRAPH91_SET_OBJECT_PRIMARY_VECTOR,
    opcodes::GRAPH91_GET_OBJECT_PROPERTY,
    opcodes::GRAPH91_GET_OBJECT_COMPOSITE_POSITION,
    opcodes::GRAPH91_ATTACH_CHILD_OBJECT,
    opcodes::GRAPH91_DETACH_CHILD_OBJECT,
    opcodes::GRAPH91_INITIALIZE_MULTILAYER_BACKGROUND,
    opcodes::GRAPH91_SELECT_MULTILAYER_LAYER,
    opcodes::GRAPH91_SET_MULTILAYER_LAYER_ENABLED,
    opcodes::GRAPH91_SET_MULTILAYER_LAYER_POSITION,
    opcodes::GRAPH91_SET_MULTILAYER_LAYER_BLEND_MODE,
    opcodes::GRAPH91_SET_MULTILAYER_LAYER_BLEND_PARAMETER,
    opcodes::GRAPH91_SET_MULTILAYER_LAYER_BITMAP,
    opcodes::GRAPH91_SET_MULTILAYER_LAYER_TRANSFORM,
    opcodes::GRAPH91_SET_MULTILAYER_LAYER_AUXILIARY_PAIR,
    opcodes::GRAPH91_SET_MULTILAYER_LAYER_SOURCE_VELOCITY,
    opcodes::GRAPH91_SET_MULTILAYER_LAYER_TRANSFORM_VELOCITY,
    opcodes::GRAPH91_SET_SPRITE_RELATION,
    opcodes::GRAPH91_CREATE_EFFECTOR,
    opcodes::GRAPH91_RELEASE_EFFECTOR,
    opcodes::GRAPH91_SET_EFFECTOR_ENABLED,
    opcodes::GRAPH91_CONFIGURE_DUAL_VECTOR_EFFECTOR,
    opcodes::GRAPH91_CONFIGURE_GRADIENT_EFFECTOR,
    opcodes::GRAPH91_CONFIGURE_RIPPLE_EFFECTOR,
    opcodes::GRAPH91_CONFIGURE_TRANSFORM_EFFECTOR,
    opcodes::GRAPH91_CONFIGURE_SURFACE_EFFECTOR,
    opcodes::GRAPH91_CREATE_LANDSCAPE,
    opcodes::GRAPH91_RELEASE_LANDSCAPE,
    opcodes::GRAPH91_HIT_TEST_LANDSCAPE,
    opcodes::GRAPH91_SET_LANDSCAPE_ENABLED,
    opcodes::GRAPH91_CONFIGURE_LANDSCAPE_OBJECT,
    opcodes::GRAPH91_SET_LANDSCAPE_CELL_SILHOUETTE,
    opcodes::GRAPH91_CONFIGURE_LANDSCAPE_PARTS,
    opcodes::GRAPH91_CONFIGURE_LANDSCAPE_MAP,
    opcodes::GRAPH91_CONFIGURE_LANDSCAPE_GUIDES,
    opcodes::GRAPH91_SET_LANDSCAPE_CELL_GUIDES,
    opcodes::GRAPH91_COPY_LANDSCAPE_PART,
    opcodes::GRAPH91_SET_LANDSCAPE_CELL_COLUMN,
    opcodes::GRAPH91_GET_LANDSCAPE_CELL_VALUE,
    opcodes::GRAPH91_COPY_LANDSCAPE_CELL_IMAGE,
    opcodes::GRAPH91_CONFIGURE_WINDOW_FONT,
    opcodes::GRAPH91_START_EXTENDED_MESSAGE,
    opcodes::GRAPH91_RENDER_WINDOW_TEXT,
    opcodes::GRAPH91_START_EXTENDED_MESSAGE_WITH_OPTION,
    opcodes::GRAPH91_RENDER_WINDOW_TEXT_WITH_STYLE_MODE,
    opcodes::GRAPH91_COUNT_TEXT_SUBSTITUTION_MATCHES,
    opcodes::GRAPH91_SET_PERSISTENT_TEXT_STYLE,
    opcodes::GRAPH91_CONFIGURE_TEXT_LAYOUT_GLOBALS,
    opcodes::GRAPH91_MEASURE_TEXT,
    opcodes::GRAPH91_DRAW_TEXT,
    opcodes::GRAPH91_DRAW_TEXT_WITH_STYLE_MODE,
    opcodes::GRAPH91_SET_WINDOW_LINE_SPACING,
    opcodes::GRAPH91_SET_WINDOW_MESSAGE_VARIANT,
    opcodes::GRAPH91_SET_WINDOW_TEXT_LAYOUT_MODE,
    opcodes::GRAPH91_SET_WINDOW_TEXT_CURSOR,
    opcodes::GRAPH91_GET_WINDOW_TEXT_CURSOR,
    opcodes::GRAPH91_IS_WINDOW_TEXT_CURSOR_AT_BOUNDARY,
    opcodes::GRAPH91_UPDATE_TEXT_SUBSTITUTION,
    opcodes::GRAPH91_REGISTER_TEXT_SUBSTITUTION_RECORDS,
    opcodes::GRAPH91_SET_TEXT_SCALE_DIVISOR,
    opcodes::GRAPH91_SET_TEXT_GLOBAL_PROPERTY,
    opcodes::GRAPH91_EXTRACT_TEXT_LABELS,
    opcodes::GRAPH91_STRIP_TEXT_MARKUP,
    opcodes::GRAPH91_CREATE_EXTENDED_ICON_INPUT_PROCESSOR,
    opcodes::GRAPH91_CONFIGURE_EXTENDED_ICON_INPUT_PROCESSOR,
    opcodes::GRAPH91_SET_EXTENDED_ICON_INPUT_ITEM_STATE,
    opcodes::GRAPH91_REGISTER_KEY_ASSIGNMENT_TABLE,
    opcodes::GRAPH91_GET_ACTIVE_KNOB_HANDLE,
    opcodes::GRAPH91_OPEN_DIRECTSHOW_MOVIE,
    opcodes::GRAPH91_START_DIRECTSHOW_MOVIE,
    opcodes::GRAPH91_CLOSE_DIRECTSHOW_MOVIE,
    opcodes::GRAPH91_SET_DIRECTSHOW_MOVIE_PAUSED,
    opcodes::GRAPH91_CREATE_FLASH_CONTROL,
    opcodes::GRAPH91_START_FLASH_CONTROL,
    opcodes::GRAPH91_CAPTURE_AND_RELEASE_FLASH_CONTROL,
    opcodes::GRAPH91_GET_MOVIE_POSITION,
    opcodes::USER_DRAW_BITMAP_TO_WINDOW,
    opcodes::USER_CENTER_MAIN_WINDOW,
    opcodes::USER_SET_MAIN_WINDOW_POSITION,
    opcodes::USER_BIND_CURSOR_OBJECT,
    opcodes::USER_SET_CURSOR_IDLE_TIMEOUT,
    opcodes::USER_QUERY_CURSOR_VISIBLE,
    opcodes::USER_SHAKE_SCREEN,
    opcodes::USER_CREATE_DEBUG_WINDOW,
    opcodes::USER_CLOSE_DEBUG_WINDOW,
    opcodes::USER_SET_DEBUG_WINDOW_VISIBLE,
    opcodes::USER_SET_DEBUG_WINDOW_TITLE,
    opcodes::USER_MOVE_DEBUG_WINDOW,
    opcodes::USER_GET_DEBUG_WINDOW_POSITION,
    opcodes::USER_CLEAR_DEBUG_WINDOW,
    opcodes::USER_DRAW_DEBUG_BITMAP,
    opcodes::USER_DRAW_DEBUG_TEXT,
    opcodes::USER_SET_DEBUG_WINDOW_CLOSE_MESSAGE,
    opcodes::USER_CREATE_EDIT_CONTROL,
    opcodes::USER_DESTROY_EDIT_CONTROL,
    opcodes::USER_SET_EDIT_FONT_SCALE,
    opcodes::USER_QUERY_EDIT_ACTIVE,
    opcodes::USER_SET_EDIT_VISIBLE,
    opcodes::USER_SET_EDIT_COLOR,
    opcodes::USER_SET_EDIT_TEXT,
    opcodes::USER_GET_EDIT_TEXT,
    opcodes::USER_SET_EDIT_HIDE_ON_ENTER,
    opcodes::USER_SET_EDIT_PRINTABLE_INPUT,
    opcodes::USER_SHOW_MESSAGE,
    opcodes::USER_SHOW_YES_NO_MESSAGE,
    opcodes::USER_SHOW_TYPED_MESSAGE,
    opcodes::USER_SET_MESSAGE_TITLE,
    opcodes::USER_SHOW_INPUT_DIALOG,
    opcodes::USER_SHOW_MULTI_FIELD_DIALOG,
    opcodes::USER_SHOW_SELECTION_DIALOG,
    opcodes::USER_SHOW_EXTENDED_DIALOG,
    opcodes::USER_SHOW_PATH_DIALOG,
    opcodes::USER_SHOW_SIX_FIELD_DIALOG,
    opcodes::USER_CREATE_MODELESS_DIALOG,
    opcodes::USER_CLOSE_MODELESS_DIALOG,
    opcodes::USER_SET_MODELESS_DIALOG_VISIBLE,
    opcodes::USER_POLL_MODELESS_DIALOG,
    opcodes::USER_INTERN_FONT_NAME,
    opcodes::USER_INTERN_FONT_NAME_WITH_OPTION,
    opcodes::USER_REGISTER_FONT_RESOURCE,
    opcodes::USER_REGISTER_ARCHIVE_FONT_RESOURCE,
    opcodes::USER_QUERY_FONT_AVAILABLE,
    opcodes::USER_QUERY_FONT_CAPABILITY,
    opcodes::USER_SET_FONT_ALIAS,
    opcodes::USER_SET_DESKTOP_WALLPAPER,
    opcodes::SOUND_QUERY_CHANNEL_COUNT,
    opcodes::SOUND_SET_BGM_PRIMARY_VOLUME,
    opcodes::SOUND_SET_SE_PRIMARY_VOLUME,
    opcodes::SOUND_LOAD_BGM_FILE,
    opcodes::SOUND_LOAD_BGM_ARCHIVE,
    opcodes::SOUND_LOAD_BGM_PAIR,
    opcodes::SOUND_CONTROL_BGM,
    opcodes::SOUND_QUERY_BGM_STATE,
    opcodes::SOUND_SET_BGM_VOLUME,
    opcodes::SOUND_SET_BGM_PAN,
    opcodes::SOUND_FADE_BGM_TO_FULL,
    opcodes::SOUND_FADE_BGM_TO_SILENCE,
    opcodes::SOUND_SET_BGM_SECONDARY_VOLUME,
    opcodes::SOUND_LOAD_SE,
    opcodes::SOUND_LOAD_SE_SCALED,
    opcodes::SOUND_RELEASE_SE,
    opcodes::SOUND_LOAD_SE_DOUBLE_RATE,
    opcodes::SOUND_PLAY_SE,
    opcodes::SOUND_STOP_SE,
    opcodes::SOUND_FADE_SE_TO_SILENCE,
    opcodes::SOUND_LOAD_SE_CUSTOM_RATE,
    opcodes::SOUND_REGISTER_SE_MEMORY,
    opcodes::SOUND_SET_SE_SECONDARY_VOLUME,
    opcodes::SOUND_GET_SE_POSITION,
    opcodes::SOUND_OPEN_CD_AUDIO,
    opcodes::SOUND_CLOSE_CD_AUDIO,
    opcodes::SOUND_PLAY_CD_TRACK,
    opcodes::SOUND_STOP_CD_AUDIO,
    opcodes::SOUND_QUERY_CD_MODE,
    opcodes::SOUND_PLAY_WAVE_ASYNC,
];

const TARGET_INFERRED_OPCODES: &[NativeOpcode] = &[];

const ABI_ONLY_DOCUMENTED_OPCODES: &[NativeOpcode] = &[];

const PORTABLE_EQUIVALENT_OPCODES: &[NativeOpcode] = &[
    opcodes::USER2_CREATE_SPLINE,
    opcodes::USER2_RELEASE_SPLINE,
    opcodes::USER2_CONFIGURE_SPLINE,
    opcodes::USER2_SAMPLE_SPLINE,
    opcodes::USER2_LOAD_BWEF_TABLE,
    opcodes::USER_QUERY_CURSOR_VISIBLE,
    opcodes::GRAPH90_REQUEST_REDRAW,
    opcodes::GRAPH90_SET_SCHEDULER_GATE,
    opcodes::GRAPH90_SET_FRAME_RATE,
    opcodes::GRAPH90_SET_CENTER,
    opcodes::GRAPH90_SET_SYNC_LOAD_HOLD,
    opcodes::GRAPH90_SET_DISPLAY_ENABLED,
    opcodes::GRAPH90_SET_DEFAULT_PRIORITY,
    opcodes::GRAPH90_SET_OBJECT_UPDATE_REDRAW_POLICY,
    opcodes::GRAPH90_SET_CURRENT_BITMAP,
    opcodes::GRAPH_SET_OBJECT_ALPHA,
    opcodes::GRAPH90_SET_BITMAP_UNBLEND_COLOR,
    opcodes::GRAPH90_RELEASE_BITMAP,
    opcodes::GRAPH90_FILL_BITMAP,
    opcodes::GRAPH90_SET_OBJECT_DRAW_ENABLED,
    opcodes::GRAPH90_SET_OBJECT_ENABLED,
    opcodes::GRAPH90_INVOKE_UNSUPPORTED_OBJECT_EXTENSION,
    opcodes::GRAPH90_SET_OBJECT_MASK_ALPHA,
    opcodes::GRAPH90_SET_OBJECT_FIXED_PARAMETER,
    opcodes::GRAPH90_SET_OBJECT_ALPHA_MULTIPLIER,
    opcodes::GRAPH90_SET_OBJECT_PRIORITY,
    opcodes::GRAPH90_SET_OBJECT_SECONDARY_OFFSET,
    opcodes::GRAPH90_SET_OBJECT_PRIMARY_OFFSET,
    opcodes::GRAPH90_GET_CURRENT_OBJECT_MODE,
    opcodes::GRAPH90_CREATE_SPRITE_OBJECT,
    opcodes::GRAPH90_SET_SPRITE_ENABLED,
    opcodes::GRAPH90_CREATE_FILTER_OBJECT,
    opcodes::GRAPH90_RELEASE_FILTER_OBJECT,
    opcodes::GRAPH90_SET_FILTER_ENABLED,
    opcodes::GRAPH90_CREATE_MAP_OBJECT,
    opcodes::GRAPH90_RELEASE_MAP_OBJECT,
    opcodes::GRAPH90_SET_MAP_ENABLED,
    opcodes::GRAPH90_CREATE_WINDOW_OBJECT,
    opcodes::GRAPH90_SET_WINDOW_COMPOSITION_ORDER,
    opcodes::GRAPH90_SET_WINDOW_DRAW_ENABLED,
    opcodes::GRAPH90_SET_WINDOW_ISOLATED_COMPOSITION,
    opcodes::GRAPH90_GET_WINDOW_VALID_REGION,
    opcodes::GRAPH90_SET_MESSAGE_CARET_FRAME_DELAY,
    opcodes::GRAPH90_SET_MESSAGE_CARET_POSITION,
    opcodes::GRAPH90_SET_TEXT_SHADOW_ENABLED,
    opcodes::SYS81_SET_CLOCK_JUMP_THRESHOLD,
    opcodes::SYS81_GET_USER_NAME,
    opcodes::SYS81_GET_COMPUTER_NAME,
    opcodes::SYS81_GET_CPU_BRAND,
    opcodes::SYS81_GET_CPU_DISPLAY_INFO,
    opcodes::SYS81_GET_PHYSICAL_MEMORY_MB,
    opcodes::SYS81_GET_ADJUSTED_DESKTOP_DIMENSIONS,
    opcodes::SYS81_IS_MAIN_WINDOW_MINIMIZED,
    opcodes::SYS81_SWAP_INPUT_BINDING,
    opcodes::SYS81_COPY_KEYBOARD_STATE,
    opcodes::SYS81_SET_KEYBOARD_POLLING_OVERRIDE,
    opcodes::SYS81_CONFIGURE_POINTER_HISTORY,
    opcodes::SYS81_COPY_POINTER_HISTORY,
    opcodes::SYS81_COPY_TOUCH_RECORDS,
    opcodes::SYS81_SET_CONTROLLER_WAKE_ENTRY,
    opcodes::SYS81_INJECT_MOUSE_CLICK,
    opcodes::SYS_SET_MESSAGE_AUXILIARY_INPUT_MASK,
    opcodes::SYS81_CONFIGURE_SCREEN_MODE,
    opcodes::SYS81_QUERY_EFFECTIVE_DISPLAY_MODE,
    opcodes::SYS81_SET_MONITOR_ADAPTER_MODE,
    opcodes::SYS81_SET_CONFIG_INPUT_MODE,
    opcodes::SYS81_CONFIGURE_LOGICAL_SCREEN_SIZE,
    opcodes::SYS81_SET_WINDOW_POSITION_OVERRIDE,
    opcodes::SYS81_SWAP_PAUSE_ON_DEACTIVATE,
    opcodes::SYS81_SET_PRINT_SCREEN_HOTKEY_CAPTURE,
    opcodes::SYS81_QUERY_PIXEL_SHADER_VERSION,
    opcodes::SYS81_UNCAUGHT_EXCEPTION_ACTIVE,
    opcodes::SYS81_WIDE_STRING_DISTANCE,
    opcodes::SYS81_SHIFT_JIS_TO_UTF16,
    opcodes::SYS81_BLOB_TABLE_OPEN,
    opcodes::SYS81_BLOB_TABLE_CLOSE,
    opcodes::SYS81_BLOB_TABLE_STORE,
    opcodes::SYS81_BLOB_TABLE_REMOVE,
    opcodes::SYS81_BLOB_TABLE_FETCH,
    opcodes::SYS81_BLOB_TABLE_ENUMERATE,
    opcodes::SYS81_UPDATE_ROLLING_HASH,
    opcodes::SYS81_MD5,
    opcodes::SYS81_CREATE_NAMED_MUTEX,
    opcodes::SYS81_RELEASE_NAMED_MUTEX,
    opcodes::SYS_SRAND,
    opcodes::SYS_RAND,
    opcodes::SYS_RAND_MAX,
    opcodes::SYS_GET_ENGINE_TICK,
    opcodes::SYS_QUERY_PERFORMANCE_COUNTER_NS,
    opcodes::SYS_GET_LOCAL_TIME,
    opcodes::SYS_GET_PHYSICAL_MEMORY,
    opcodes::SYS_QUERY_WINDOW_ACTIVE,
    opcodes::SYS_MOVE_FILE,
    opcodes::SYS_CREATE_DIRECTORY,
    opcodes::SYS_REMOVE_DIRECTORY,
    opcodes::SYS_WRITE_FILE_BYTES,
    opcodes::SYS_SET_ADDITIONAL_RESOURCE_SEARCH,
    opcodes::SYS_PREPEND_RESOURCE_SEARCH_PATH,
    opcodes::SYS_REGISTER_COMPOSITE_ARCHIVE,
    opcodes::SYS_RTC_NOOP,
    opcodes::SYS_CURRENT_THREAD_ID,
    opcodes::SYS_THREAD_EXISTS,
    opcodes::SYS_ENQUEUE_MESSAGE,
    opcodes::SYS_DEQUEUE_MESSAGE,
    opcodes::SYS_ENQUEUE_MESSAGE_ARRAY,
    opcodes::SYS_DEQUEUE_MESSAGE_ARRAY,
    opcodes::SYS_SET_SYSTEM_WAIT_STATE,
    opcodes::SYS_ARM_NEXT_BINARY_ASYNC,
    opcodes::SYS_QUERY_DROPPED_FILE,
    opcodes::SYS_SET_SAVE_DATA_INTEGRITY,
    opcodes::SYS_SET_AND_QUERY_THREAD_TIMER,
    opcodes::SYS_WRITE_GLOBAL_DATA_BLOCK,
    opcodes::SYS_READ_GLOBAL_DATA_BLOCK,
    opcodes::SYS_RESET_STRUCTURED_HISTORY,
    opcodes::SYS_STRUCTURED_HISTORY_COUNT,
    opcodes::SYS_APPEND_STRUCTURED_HISTORY_FIELDS,
    opcodes::SYS_READ_STRUCTURED_HISTORY,
    opcodes::SYS_APPEND_STRUCTURED_HISTORY_RECORD,
    opcodes::SYS_READ_STRUCTURED_HISTORY_EXTENDED,
    opcodes::SYS_INDEXED_RECORD_OPEN,
    opcodes::SYS_INDEXED_RECORD_CLOSE,
    opcodes::SYS_INDEXED_RECORD_COUNT,
    opcodes::SYS_INDEXED_RECORD_PUSH,
    opcodes::SYS_INDEXED_RECORD_LOAD,
    opcodes::SYS_INDEXED_RECORD_REMOVE,
    opcodes::SYS_POLL_QUEUED_EVENT,
    opcodes::SYS_POST_QUEUED_EVENT,
    opcodes::SYS_SET_SYSTEM_MODE_FLAG,
    opcodes::SYS_CREATE_EXCLUSION_SECTION,
    opcodes::SYS_DELETE_EXCLUSION_SECTION,
    opcodes::SYS_LEAVE_EXCLUSION_SECTION,
    opcodes::SYS_QUERY_EXCLUSION_SECTION,
    opcodes::SYS_INTERN_RESOURCE_NAME,
    opcodes::SYS_RESOURCE_NAME_EXISTS,
    opcodes::SYS_CREATE_OR_RESIZE_READ_FLAG_TABLE,
    opcodes::SYS_SET_READ_FLAG_BIT,
    opcodes::SYS_SET_READ_FLAG_RANGE,
    opcodes::SYS_QUERY_READ_FLAG_BIT,
    opcodes::SYS_COPY_INDEXED_NAMESPACE_RECORD,
    opcodes::SYS_DECODE_SDC_BUFFER,
    opcodes::SYS_DECODE_STRUCT_ARRAY,
    opcodes::SYS_RECORD_TABLE_OPEN,
    opcodes::SYS_RECORD_TABLE_CLOSE,
    opcodes::SYS_RECORD_TABLE_INSERT,
    opcodes::SYS_RECORD_TABLE_REMOVE,
    opcodes::SYS_RECORD_TABLE_FETCH,
    opcodes::SYS_CLEAR_STRING_NAMESPACES,
    opcodes::SYS_STRING_NAMESPACE_COUNT,
    opcodes::SYS_REPLACE_STRING_NAMESPACE,
    opcodes::SYS_SERIALIZE_STRING_NAMESPACE,
    opcodes::SYS_INTERN_STRING_NAMESPACE,
    opcodes::SYS_STRING_NAMESPACE_ENTRY_LENGTH,
    opcodes::SYS_WRITE_GAME_ID,
    opcodes::SYS_HASH_FILE,
    opcodes::SYS_SET_UNINSTALLER_PRODUCT,
    opcodes::SYS_INSTALLER_FEATURE_AVAILABLE,
    opcodes::SYS_YIELD,
    opcodes::GRAPH90_SET_ITEM_SELECTION_HIGHLIGHT_STYLES,
    opcodes::GRAPH90_SET_ITEM_SELECTION_INPUT_MASK,
    opcodes::GRAPH90_SET_ITEM_SELECTION_NAVIGATION_PARAMETERS,
    opcodes::GRAPH90_SET_ITEM_SELECTION_COLUMN_LAYOUT,
    opcodes::GRAPH90_SET_INTERACTIVE_PROCEDURE_POLL_GATE,
    opcodes::GRAPH90_FLIP_BITMAP,
    opcodes::GRAPH90_REGISTER_BG_RESOURCE_DATA,
    opcodes::GRAPH90_REGISTER_TONE_CURVE,
    opcodes::GRAPH90_CREATE_KNOB_OBJECT,
    opcodes::GRAPH90_RELEASE_KNOB_OBJECT,
    opcodes::GRAPH90_SET_KNOB_ENABLED,
    opcodes::GRAPH90_SET_KNOB_BASE_POSITION,
    opcodes::GRAPH90_SET_KNOB_POSITION,
    opcodes::GRAPH90_GET_KNOB_POSITION,
    opcodes::GRAPH90_SET_KNOB_MOVEMENT_PRECISION,
    opcodes::GRAPH90_SET_KNOB_MOVEMENT_RANGE,
    opcodes::GRAPH90_SET_KNOB_RELATIVE_MODE,
    opcodes::GRAPH90_SWAP_KNOB_INPUT_MODE,
    opcodes::GRAPH90_CREATE_GROUP_OBJECT,
    opcodes::GRAPH90_RELEASE_GROUP_OBJECT,
    opcodes::GRAPH90_SET_GROUP_ENABLED,
    opcodes::GRAPH90_CONFIGURE_GROUP_OBJECT,
    opcodes::GRAPH90_ADD_OBJECT_TO_GROUP,
    opcodes::GRAPH90_REMOVE_OBJECT_FROM_GROUP,
    opcodes::GRAPH90_CLOSE_DIRECTSHOW_MOVIE,
    opcodes::GRAPH90_IS_DIRECTSHOW_MOVIE_PLAYING,
    opcodes::GRAPH90_SET_DIRECTSHOW_MOVIE_VOLUME,
    opcodes::GRAPH90_RELEASE_BURIKO_MOVIE_RESOURCE,
    opcodes::GRAPH90_ATTACH_BURIKO_MOVIE_RESOURCE,
    opcodes::GRAPH90_CLEAR_SPRITE_TARGETS,
    opcodes::GRAPH90_REGISTER_SPRITE_TARGET,
    opcodes::GRAPH90_UNREGISTER_SPRITE_TARGET,
    opcodes::GRAPH91_CACHE_BINARY_RESOURCE,
    opcodes::GRAPH91_SET_GLOBAL_DISPLAY_OFFSET,
    opcodes::GRAPH91_GENERATE_AFFINE_DISPLACEMENT_MAP,
    opcodes::GRAPH91_GENERATE_RIPPLE_DISPLACEMENT_MAP,
    opcodes::GRAPH91_GENERATE_RADIAL_LENS_DISPLACEMENT_MAP,
    opcodes::GRAPH91_REPLACE_BITMAP_RGB_PRESERVE_ALPHA,
    opcodes::GRAPH91_CLONE_BITMAP,
    opcodes::GRAPH91_SET_OBJECT_SUPPRESSED,
    opcodes::GRAPH91_GET_OBJECT_COMPOSITE_POSITION,
    opcodes::GRAPH91_ATTACH_CHILD_OBJECT,
    opcodes::GRAPH91_DETACH_CHILD_OBJECT,
    opcodes::GRAPH91_CREATE_EFFECTOR,
    opcodes::GRAPH91_RELEASE_EFFECTOR,
    opcodes::GRAPH91_SET_EFFECTOR_ENABLED,
    opcodes::GRAPH91_CREATE_LANDSCAPE,
    opcodes::GRAPH91_RELEASE_LANDSCAPE,
    opcodes::GRAPH91_SET_LANDSCAPE_ENABLED,
    opcodes::GRAPH91_SET_WINDOW_LINE_SPACING,
    opcodes::GRAPH91_SET_WINDOW_MESSAGE_VARIANT,
    opcodes::GRAPH91_SET_WINDOW_TEXT_LAYOUT_MODE,
    opcodes::GRAPH91_SET_WINDOW_TEXT_CURSOR,
    opcodes::GRAPH91_GET_WINDOW_TEXT_CURSOR,
    opcodes::GRAPH91_IS_WINDOW_TEXT_CURSOR_AT_BOUNDARY,
    opcodes::GRAPH91_UPDATE_TEXT_SUBSTITUTION,
    opcodes::GRAPH91_REGISTER_TEXT_SUBSTITUTION_RECORDS,
    opcodes::GRAPH91_SET_TEXT_SCALE_DIVISOR,
    opcodes::GRAPH91_SET_TEXT_GLOBAL_PROPERTY,
    opcodes::GRAPH91_EXTRACT_TEXT_LABELS,
    opcodes::GRAPH91_STRIP_TEXT_MARKUP,
    opcodes::GRAPH91_REGISTER_KEY_ASSIGNMENT_TABLE,
    opcodes::GRAPH91_CLOSE_DIRECTSHOW_MOVIE,
    opcodes::GRAPH91_SET_DIRECTSHOW_MOVIE_PAUSED,
    opcodes::GRAPH92_CONFIGURE_COMPACT_WAVE_TABLE,
    opcodes::GRAPH92_CONFIGURE_WAVE_TABLE,
    opcodes::GRAPH92_GENERATE_RADIAL_VECTOR_MAP,
    opcodes::GRAPH92_GENERATE_AXIS_VECTOR_MAP,
    opcodes::GRAPH92_SET_BITMAP_AUXILIARY_PAIR,
    opcodes::GRAPH92_REPLACE_BITMAP_COLOR,
    opcodes::GRAPH92_GET_BITMAP_AUXILIARY_PAIR,
    opcodes::GRAPH_CONVERT_BITMAP_TO_ALPHA_DESCRIPTOR,
    opcodes::GRAPH92_INVERT_ALPHA_BITMAP,
    opcodes::GRAPH92_GET_TEXT_OUTPUT_PAIR,
    opcodes::GRAPH92_DRAIN_TEXT_FRAGMENT_RECORDS,
    opcodes::GRAPH92_SET_TEXT_RENDER_OVERRIDE,
    opcodes::SOUND_QUERY_CHANNEL_COUNT,
    opcodes::SYS_WAIT_TIMING_EX,
    opcodes::GRAPH90_BLIT_BITMAP,
    opcodes::GRAPH90_CREATE_BITMAP,
    opcodes::GRAPH90_QUERY_BITMAP_INFO,
    opcodes::GRAPH90_RELEASE_SPRITE_OBJECT,
    opcodes::SYS_RESET_INPUT_CONFIGURATION,
    opcodes::SYS_QUERY_KEY_DOWN,
    opcodes::SYS_SUM_INPUT_DESCRIPTOR_STATE,
    opcodes::SYS_INPUT_MESSAGE_SERIAL,
    opcodes::SYS_SET_INPUT_MASTER_GATE,
    opcodes::SYS_SET_INPUT_LATCHED_STATE,
    opcodes::SYS_SAMPLE_CONFIGURED_INPUT,
    opcodes::SYS_QUERY_CONFIGURED_INPUT_GATE,
    opcodes::SYS_REGISTER_INPUT_SCOPE,
    opcodes::SYS_QUERY_AND_UNREGISTER_INPUT_SCOPE,
    opcodes::SYS_REGISTER_INPUT_CLASS_DESCRIPTORS,
    opcodes::SYS_QUERY_INPUT_CLASS_LEVEL,
    opcodes::SYS_SET_MOUSE_BUTTON_MAPPING_MODE,
    opcodes::GRAPH90_LOAD_BITMAP,
    opcodes::GRAPH90_CREATE_BITMAP_REGION,
    opcodes::GRAPH90_CONFIGURE_SPRITE_SINGLE_BITMAP,
    opcodes::USER_SHOW_MESSAGE,
    opcodes::USER_SHOW_YES_NO_MESSAGE,
    opcodes::USER_SHOW_TYPED_MESSAGE,
    opcodes::USER_SET_MESSAGE_TITLE,
    opcodes::GRAPH90_START_OBJECT_CONTROL,
    opcodes::GRAPH90_START_NODE_CONTROL,
    opcodes::GRAPH90_START_OBJECT_CONTROL_EX,
    opcodes::GRAPH90_START_NODE_CONTROL_EX,
    opcodes::SYS_FILE_EXISTS,
    opcodes::SYS_DELETE_FILE,
    opcodes::GRAPH90_REPLACE_SPRITE_BITMAP,
    opcodes::GRAPH90_APPLY_ICON_INPUT_LAYOUT_EX,
    opcodes::GRAPH92_DRAW_BITMAP_TEXT,
    opcodes::GRAPH91_CONFIGURE_WINDOW_FONT,
    opcodes::GRAPH90_SET_OBJECT_POSITION,
    opcodes::GRAPH90_RELEASE_WINDOW_OBJECT,
    opcodes::SYS_READ_CURSOR_POINT,
    opcodes::GRAPH90_RELEASE_ICON_INPUT_PROCESSOR,
    opcodes::SYS_INVOKE_THREAD_CALLBACK,
    opcodes::SYS_SET_REGISTERED_OBJECT_STATE,
    opcodes::SYS_GET_REGISTERED_OBJECT_STATE,
    opcodes::SYS_QUEUE_REGISTERED_OBJECT_MESSAGE,
    opcodes::GRAPH90_CONFIGURE_CURRENT_OBJECT_SPRITE_MASK,
    opcodes::GRAPH90_SET_WINDOW_VALID_REGION,
    opcodes::SYS_SWITCH_PROGRAM,
    opcodes::GRAPH90_SET_CURRENT_OBJECT_RENDER_CONTROLS,
    opcodes::GRAPH_RENDER_OBJECT_TO_BITMAP,
    opcodes::GRAPH90_CONFIGURE_WINDOW_OBJECT,
    opcodes::GRAPH_BIND_BITMAP_TO_SURFACE,
    opcodes::GRAPH90_CREATE_WORK_BITMAP,
    opcodes::GRAPH90_CREATE_PRIORITIZED_WORK_BITMAP,
    opcodes::SOUND_PLAY_SE,
    opcodes::SOUND_GET_SE_POSITION,
    opcodes::GRAPH91_CREATE_EXTENDED_ICON_INPUT_PROCESSOR,
    opcodes::GRAPH91_CONFIGURE_EXTENDED_ICON_INPUT_PROCESSOR,
    opcodes::GRAPH90_GET_ICON_INPUT_STATE,
    opcodes::GRAPH90_POP_ICON_INPUT_EVENT,
    opcodes::SOUND_CONTROL_BGM,
    opcodes::SOUND_SET_BGM_VOLUME,
    opcodes::GRAPH91_SET_OBJECT_FIXED_POSITION,
    opcodes::GRAPH91_SET_OBJECT_SECONDARY_VECTOR,
    opcodes::GRAPH91_SET_OBJECT_PRIMARY_VECTOR,
    opcodes::GRAPH91_COUNT_TEXT_SUBSTITUTION_MATCHES,
    opcodes::GRAPH90_REFRESH_SPRITE_OBJECT,
    opcodes::GRAPH90_START_OBJECT_MOTION_CONTROL,
    opcodes::GRAPH90_CONFIGURE_SPRITE_DUAL_BITMAP,
    opcodes::SYS_CONFIGURE_CURSOR_MOTION,
    opcodes::SOUND_STOP_SE,
    opcodes::SOUND_RELEASE_SE,
    opcodes::SYS_SET_PERFORMANCE_PROFILING,
    opcodes::SYS_READ_PERFORMANCE_METRIC,
    opcodes::SYS_QUERY_PRESENTATION_STATE,
    opcodes::SYS_COPY_GRAPHICS_CAPABILITIES,
    opcodes::SYS_QUERY_GRAPHICS_MEMORY_METRIC,
    opcodes::SYS_QUERY_WINDOW_MINIMIZE_LATCH,
    opcodes::SYS_MINIMIZE_WINDOW,
    opcodes::SYS_SET_THREAD_TIMER,
    opcodes::SYS_WAIT_THREAD_TIMER,
    opcodes::SYS_QUERY_INPUT_EVENT_BITS,
    opcodes::SYS_QUERY_SCOPED_INPUT_EVENT,
    opcodes::SYS_COUNT_FILES,
    opcodes::SYS_ENUMERATE_FILES,
    opcodes::SYS_ENUMERATE_DIRECTORIES,
    opcodes::SYS_DIRECTORY_EXISTS,
    opcodes::SYS_SPLIT_PATH,
    opcodes::SYS_GET_FILE_ATTRIBUTES,
    opcodes::SYS_SET_FILE_ATTRIBUTES,
    opcodes::SYS_COPY_FILE,
    opcodes::SYS_READ_FILE_BYTES,
    opcodes::SYS_READ_FILE_RANGE,
    opcodes::SYS_FILE_SIZE,
    opcodes::SYS81_COPY_COORDINATE_SLOT,
    opcodes::SYS81_GET_OS_VERSION,
    opcodes::SYS81_REGISTER_TOUCH_INPUT,
    opcodes::SYS81_QUERY_CONTROLLER_STATE,
    opcodes::SYS81_SET_BICUBIC_SHADER,
    opcodes::SYS81_LAUNCH_PROCESS_WAIT,
    opcodes::GRAPH90_INITIALIZE_BITMAP_MEMORY_MANAGER,
    opcodes::GRAPH90_SET_RASTER_FORMAT_MODE,
    opcodes::GRAPH90_REGISTER_FONT,
    opcodes::GRAPH90_CREATE_BITMAP_FROM_PIXELS,
    opcodes::GRAPH90_COPY_BITMAP_PIXELS,
    opcodes::GRAPH90_VALIDATE_BITMAP_FORMAT,
    opcodes::GRAPH90_SYNTHESIZE_BITMAP,
    opcodes::GRAPH90_COMPOSITE_BITMAPS,
    opcodes::GRAPH90_COPY_BITMAP,
    opcodes::GRAPH90_SCALE_BITMAP_REGION,
    opcodes::GRAPH90_TRANSFORM_BITMAP,
    opcodes::GRAPH90_BLIT_BITMAP_REGION,
    opcodes::GRAPH90_START_SPECIAL_OBJECT_CONTROL,
    opcodes::GRAPH90_START_SPLINE_OBJECT_CONTROL,
    opcodes::GRAPH90_START_SHAKE_OBJECT_CONTROL,
    opcodes::GRAPH90_SET_OBJECT_PROPERTY,
    opcodes::GRAPH90_SET_OBJECT_HIT_MASK_BITMAP,
    opcodes::GRAPH90_HIT_TEST_OBJECT_AT_POINTER,
    opcodes::GRAPH90_SET_CURRENT_OBJECT_BITMAP,
    opcodes::GRAPH90_CONFIGURE_CURRENT_OBJECT_DUAL_BITMAP,
    opcodes::GRAPH90_CONFIGURE_CURRENT_OBJECT_QUAD_BITMAP,
    opcodes::GRAPH90_CONFIGURE_CURRENT_OBJECT_FRAME_TABLE,
    opcodes::GRAPH90_CONFIGURE_CURRENT_OBJECT_RESOURCE_TRIPLET,
    opcodes::GRAPH90_CONFIGURE_CURRENT_OBJECT_BITMAP_MODE,
    opcodes::GRAPH90_CONFIGURE_CURRENT_OBJECT_VM_EFFECT,
    opcodes::GRAPH90_CONFIGURE_CURRENT_OBJECT_BITMAP_SIZE_POSITION,
    opcodes::GRAPH90_CONFIGURE_CURRENT_OBJECT_BITMAP_SIZE,
    opcodes::GRAPH90_CONFIGURE_CURRENT_OBJECT_BLIT_SOURCES,
    opcodes::GRAPH90_SET_SPRITE_AUX_BITMAP,
    opcodes::GRAPH90_CONFIGURE_SPRITE_SCALED_BITMAP,
    opcodes::GRAPH90_CONFIGURE_SPRITE_MASKED_BITMAP,
    opcodes::GRAPH90_CONFIGURE_SPRITE_VM_EFFECT,
    opcodes::GRAPH90_CONFIGURE_SPRITE_TRANSFORM_MODE5,
    opcodes::GRAPH90_CONFIGURE_SPRITE_TRANSFORM_MODE6,
    opcodes::GRAPH90_CONFIGURE_FILTER,
    opcodes::GRAPH90_CONFIGURE_FILTER_WITH_MASK,
    opcodes::GRAPH90_CONFIGURE_MAP_OBJECT,
    opcodes::GRAPH90_INITIALIZE_MAP_GRID,
    opcodes::GRAPH90_SET_MAP_VIEWPORT,
    opcodes::GRAPH90_REPLACE_MAP_TILE_ID,
    opcodes::GRAPH90_START_WINDOW_MESSAGE_PROCEDURE,
    opcodes::GRAPH_SET_MESSAGE_INPUT_SCOPE,
    opcodes::GRAPH_SET_MESSAGE_INPUT_FILTER,
    opcodes::GRAPH_SET_GLYPH_REVEAL_DELAY,
    opcodes::GRAPH_SET_TEXT_REVEAL_ANIMATION,
    opcodes::GRAPH_SET_TEXT_SETTLE_ANIMATION,
    opcodes::GRAPH_SET_TEXT_AUTO_ADVANCE,
    opcodes::GRAPH90_CONFIGURE_MESSAGE_CARET_FRAMES,
    opcodes::GRAPH_SET_MESSAGE_START_DELAY,
    opcodes::GRAPH90_SET_TEXT_SHADOW_PARAMETERS,
    opcodes::GRAPH90_REGISTER_FULLWIDTH_GLYPH_STRIP,
    opcodes::GRAPH_SET_MESSAGE_INPUT_FORCES_COMPLETION,
    opcodes::GRAPH90_START_ITEM_SELECTION,
    opcodes::GRAPH90_DRAW_ITEM_SELECTION_GRID,
    opcodes::GRAPH90_START_ITEM_SELECTION_EX,
    opcodes::GRAPH90_START_ITEM_SELECTION_EX_BLINK,
    opcodes::GRAPH90_START_ICON_SELECTION,
    opcodes::GRAPH90_START_ICON_SELECTION_EX,
    opcodes::GRAPH90_DRAW_ICON_BATCH,
    opcodes::GRAPH90_DRAW_EXTENDED_ICON_BATCH,
    opcodes::GRAPH90_APPLY_ICON_INPUT_LAYOUT,
    opcodes::GRAPH90_CREATE_ICON_INPUT_PROCESSOR,
    opcodes::GRAPH90_CONFIGURE_ICON_INPUT_PROCESSOR,
    opcodes::GRAPH90_GET_ICON_INPUT_CURRENT_GROUP,
    opcodes::GRAPH90_GET_ICON_INPUT_SELECTIONS,
    opcodes::GRAPH90_LOAD_BG_BITMAP_RESOURCE,
    opcodes::GRAPH90_DOWNSAMPLE_BITMAP_HALF,
    opcodes::GRAPH90_LOAD_CACHED_BG_BITMAP,
    opcodes::GRAPH90_TRANSFORM_BITMAP_GENERAL,
    opcodes::GRAPH90_SCALE_BITMAP_ASPECT_FIT,
    opcodes::GRAPH90_TAKE_KNOB_VERTICAL_EVENT,
    opcodes::GRAPH90_TAKE_CHANGED_KNOB_HANDLE,
    opcodes::GRAPH90_WATCH_KNOB_OBJECT,
    opcodes::GRAPH90_UNWATCH_KNOB_OBJECT,
    opcodes::GRAPH90_OPEN_DIRECTSHOW_MOVIE,
    opcodes::GRAPH90_LOAD_BURIKO_MOVIE_RESOURCE,
    opcodes::GRAPH90_DECODE_BURIKO_MOVIE_FRAME,
    opcodes::GRAPH90_HIT_TEST_SPRITE_TARGETS,
    opcodes::GRAPH90_GET_SPRITE_TARGET_STATE,
    opcodes::GRAPH91_SET_SCRIPT_BITMAP_CONTEXT_BINDING_ENABLED,
    opcodes::GRAPH91_SET_GLYPH_COVERAGE_MODE,
    opcodes::GRAPH91_SET_FONT_PITCH_DETECTION_ENABLED,
    opcodes::GRAPH91_REGISTER_NAMED_FONT_TRANSFORM,
    opcodes::GRAPH91_CONFIGURE_NATIVE_FONT,
    opcodes::GRAPH91_GENERATE_RANDOM_DISPLACEMENT_MAP,
    opcodes::GRAPH91_GENERATE_PERSPECTIVE_BEND_MAP,
    opcodes::GRAPH91_GENERATE_CURVATURE_DISPLACEMENT_MAP,
    opcodes::GRAPH91_GENERATE_SINE_DISPLACEMENT_MAP,
    opcodes::GRAPH91_GENERATE_RADIAL_WARP_DISPLACEMENT_MAP,
    opcodes::GRAPH91_GET_OBJECT_PROPERTY,
    opcodes::GRAPH91_INITIALIZE_MULTILAYER_BACKGROUND,
    opcodes::GRAPH91_SELECT_MULTILAYER_LAYER,
    opcodes::GRAPH91_SET_MULTILAYER_LAYER_ENABLED,
    opcodes::GRAPH91_SET_MULTILAYER_LAYER_POSITION,
    opcodes::GRAPH91_SET_MULTILAYER_LAYER_BLEND_MODE,
    opcodes::GRAPH91_SET_MULTILAYER_LAYER_BLEND_PARAMETER,
    opcodes::GRAPH91_SET_MULTILAYER_LAYER_BITMAP,
    opcodes::GRAPH91_SET_MULTILAYER_LAYER_TRANSFORM,
    opcodes::GRAPH91_SET_MULTILAYER_LAYER_AUXILIARY_PAIR,
    opcodes::GRAPH91_SET_MULTILAYER_LAYER_SOURCE_VELOCITY,
    opcodes::GRAPH91_SET_MULTILAYER_LAYER_TRANSFORM_VELOCITY,
    opcodes::GRAPH91_SET_SPRITE_RELATION,
    opcodes::GRAPH91_CONFIGURE_DUAL_VECTOR_EFFECTOR,
    opcodes::GRAPH91_CONFIGURE_GRADIENT_EFFECTOR,
    opcodes::GRAPH91_CONFIGURE_RIPPLE_EFFECTOR,
    opcodes::GRAPH91_CONFIGURE_TRANSFORM_EFFECTOR,
    opcodes::GRAPH91_CONFIGURE_SURFACE_EFFECTOR,
    opcodes::GRAPH91_HIT_TEST_LANDSCAPE,
    opcodes::GRAPH91_CONFIGURE_LANDSCAPE_OBJECT,
    opcodes::GRAPH91_SET_LANDSCAPE_CELL_SILHOUETTE,
    opcodes::GRAPH91_CONFIGURE_LANDSCAPE_PARTS,
    opcodes::GRAPH91_CONFIGURE_LANDSCAPE_MAP,
    opcodes::GRAPH91_CONFIGURE_LANDSCAPE_GUIDES,
    opcodes::GRAPH91_SET_LANDSCAPE_CELL_GUIDES,
    opcodes::GRAPH91_COPY_LANDSCAPE_PART,
    opcodes::GRAPH91_SET_LANDSCAPE_CELL_COLUMN,
    opcodes::GRAPH91_GET_LANDSCAPE_CELL_VALUE,
    opcodes::GRAPH91_COPY_LANDSCAPE_CELL_IMAGE,
    opcodes::GRAPH91_START_EXTENDED_MESSAGE,
    opcodes::GRAPH91_RENDER_WINDOW_TEXT,
    opcodes::GRAPH91_START_EXTENDED_MESSAGE_WITH_OPTION,
    opcodes::GRAPH91_RENDER_WINDOW_TEXT_WITH_STYLE_MODE,
    opcodes::GRAPH91_SET_PERSISTENT_TEXT_STYLE,
    opcodes::GRAPH91_CONFIGURE_TEXT_LAYOUT_GLOBALS,
    opcodes::GRAPH91_MEASURE_TEXT,
    opcodes::GRAPH91_DRAW_TEXT,
    opcodes::GRAPH91_DRAW_TEXT_WITH_STYLE_MODE,
    opcodes::GRAPH91_SET_EXTENDED_ICON_INPUT_ITEM_STATE,
    opcodes::GRAPH91_GET_ACTIVE_KNOB_HANDLE,
    opcodes::GRAPH91_OPEN_DIRECTSHOW_MOVIE,
    opcodes::GRAPH91_START_DIRECTSHOW_MOVIE,
    opcodes::GRAPH91_CREATE_FLASH_CONTROL,
    opcodes::GRAPH91_START_FLASH_CONTROL,
    opcodes::GRAPH91_CAPTURE_AND_RELEASE_FLASH_CONTROL,
    opcodes::GRAPH91_GET_MOVIE_POSITION,
    opcodes::GRAPH92_READ_BITMAP_PIXEL_VALUE,
    opcodes::GRAPH92_COMPOSE_BITMAP_ALPHA_AT_OFFSET,
    opcodes::GRAPH92_DRAW_BITMAP_TEXT_MEASURE,
    opcodes::GRAPH92_DRAW_WRAPPED_BITMAP_TEXT,
    opcodes::GRAPH92_SET_TEXT_OBJECT_VALUE,
    opcodes::GRAPH92_COMPOSITE_RESOURCE_INTO_TEXT_OBJECT,
    opcodes::GRAPH92_SET_TEXT_OBJECT_AUXILIARY_VALUE,
    opcodes::GRAPH92_SET_TEXT_OBJECT_UPDATE_FLAG,
    opcodes::GRAPH92_APPLY_EFFECT_RESOURCE,
    opcodes::GRAPH92_RESET_TEXT_OBJECT,
    opcodes::GRAPH92_START_STYLED_MESSAGE,
    opcodes::GRAPH92_DRAW_FORMATTED_TEXT,
    opcodes::GRAPH_SET_PERSISTENT_TEXT_STYLE,
    opcodes::GRAPH92_CONFIGURE_TEXT_BITMAP_SLOT,
    opcodes::GRAPH92_RENDER_TEXT,
    opcodes::GRAPH92_CONFIGURE_FONT_OVERRIDE,
    opcodes::GRAPH92_OPEN_GLOBAL_DIRECTSHOW_MOVIE,
    opcodes::GRAPH92_OPEN_BITMAP_DIRECTSHOW_MOVIE,
    opcodes::GRAPH92_SEEK_BITMAP_DIRECTSHOW_MOVIE,
    opcodes::GRAPH92_GET_BITMAP_DIRECTSHOW_MOVIE_POSITION,
    opcodes::GRAPH92_SET_BITMAP_DIRECTSHOW_MOVIE_VOLUME,
    opcodes::SOUND_LOAD_BGM_FILE,
    opcodes::SOUND_LOAD_BGM_ARCHIVE,
    opcodes::SOUND_LOAD_BGM_PAIR,
    opcodes::SOUND_QUERY_BGM_STATE,
    opcodes::SOUND_SET_BGM_PAN,
    opcodes::SOUND_FADE_BGM_TO_FULL,
    opcodes::SOUND_FADE_BGM_TO_SILENCE,
    opcodes::SOUND_LOAD_SE,
    opcodes::SOUND_LOAD_SE_SCALED,
    opcodes::SOUND_LOAD_SE_DOUBLE_RATE,
    opcodes::SOUND_FADE_SE_TO_SILENCE,
    opcodes::SOUND_LOAD_SE_CUSTOM_RATE,
    opcodes::SOUND_REGISTER_SE_MEMORY,
    opcodes::SOUND_OPEN_CD_AUDIO,
    opcodes::SOUND_CLOSE_CD_AUDIO,
    opcodes::SOUND_PLAY_CD_TRACK,
    opcodes::SOUND_STOP_CD_AUDIO,
    opcodes::SOUND_PLAY_WAVE_ASYNC,
    opcodes::USER_DRAW_BITMAP_TO_WINDOW,
    opcodes::USER_CENTER_MAIN_WINDOW,
    opcodes::USER_SET_MAIN_WINDOW_POSITION,
    opcodes::USER_BIND_CURSOR_OBJECT,
    opcodes::USER_SET_CURSOR_IDLE_TIMEOUT,
    opcodes::USER_SHAKE_SCREEN,
    opcodes::USER_CREATE_DEBUG_WINDOW,
    opcodes::USER_CLOSE_DEBUG_WINDOW,
    opcodes::USER_SET_DEBUG_WINDOW_VISIBLE,
    opcodes::USER_SET_DEBUG_WINDOW_TITLE,
    opcodes::USER_MOVE_DEBUG_WINDOW,
    opcodes::USER_GET_DEBUG_WINDOW_POSITION,
    opcodes::USER_CLEAR_DEBUG_WINDOW,
    opcodes::USER_DRAW_DEBUG_BITMAP,
    opcodes::USER_DRAW_DEBUG_TEXT,
    opcodes::USER_SET_DEBUG_WINDOW_CLOSE_MESSAGE,
    opcodes::USER_CREATE_EDIT_CONTROL,
    opcodes::USER_DESTROY_EDIT_CONTROL,
    opcodes::USER_SET_EDIT_FONT_SCALE,
    opcodes::USER_QUERY_EDIT_ACTIVE,
    opcodes::USER_SET_EDIT_VISIBLE,
    opcodes::USER_SET_EDIT_COLOR,
    opcodes::USER_SET_EDIT_TEXT,
    opcodes::USER_GET_EDIT_TEXT,
    opcodes::USER_SET_EDIT_HIDE_ON_ENTER,
    opcodes::USER_SET_EDIT_PRINTABLE_INPUT,
    opcodes::USER_SHOW_INPUT_DIALOG,
    opcodes::USER_SHOW_MULTI_FIELD_DIALOG,
    opcodes::USER_SHOW_SELECTION_DIALOG,
    opcodes::USER_SHOW_EXTENDED_DIALOG,
    opcodes::USER_SHOW_PATH_DIALOG,
    opcodes::USER_SHOW_SIX_FIELD_DIALOG,
    opcodes::USER_CREATE_MODELESS_DIALOG,
    opcodes::USER_CLOSE_MODELESS_DIALOG,
    opcodes::USER_SET_MODELESS_DIALOG_VISIBLE,
    opcodes::USER_POLL_MODELESS_DIALOG,
    opcodes::USER_INTERN_FONT_NAME,
    opcodes::USER_INTERN_FONT_NAME_WITH_OPTION,
    opcodes::USER_REGISTER_FONT_RESOURCE,
    opcodes::USER_REGISTER_ARCHIVE_FONT_RESOURCE,
    opcodes::USER_QUERY_FONT_AVAILABLE,
    opcodes::USER_QUERY_FONT_CAPABILITY,
    opcodes::USER_SET_FONT_ALIAS,
    opcodes::USER_SET_DESKTOP_WALLPAPER,
    opcodes::USER2_CREATE_PARTICLE_SCREEN,
    opcodes::USER2_RELEASE_PARTICLE_SCREEN,
    opcodes::USER2_SET_PARTICLE_SCREEN_ENABLED,
    opcodes::USER2_CONFIGURE_PARTICLE_SCREEN_DISPLAY,
    opcodes::USER2_CONFIGURE_PARTICLE_FRAME_TABLES,
    opcodes::USER2_COMMIT_PARTICLE_SCREEN,
    opcodes::USER2_SET_PARTICLE_AUTO_UPDATE,
    opcodes::USER2_SET_PARTICLE_CAPACITY,
    opcodes::USER2_CONFIGURE_PARTICLE_EMITTER_TRANSFORM,
    opcodes::USER2_SET_PARTICLE_EMISSION_PERCENT,
    opcodes::USER2_ADVANCE_PARTICLE_SCREEN,
    opcodes::USER2_RESET_PARTICLE_SCREEN,
    opcodes::USER2_CONFIGURE_PARTICLE_EMITTER,
    opcodes::USER2_CONFIGURE_PARTICLE_ANIMATION_BANK,
    opcodes::USER2_LOAD_PARTICLE_ANIMATION_FRAMES,
    opcodes::USER2_COMMIT_PARTICLE_ANIMATION_FRAMES,
    opcodes::USER2_SET_PARTICLE_INTERPOLATION_MODE,
    opcodes::USER2_SET_PARTICLE_STYLE0,
    opcodes::USER2_DEFINE_PARTICLE_STYLE0,
    opcodes::USER2_CONFIGURE_PARTICLE_STYLE0,
    opcodes::USER2_SET_PARTICLE_STYLE1,
    opcodes::USER2_CONFIGURE_PARTICLE_STYLE1,
    opcodes::USER2_DEFINE_PARTICLE_STYLE1,
    opcodes::USER2_CONFIGURE_PARTICLE_ADVANCED_STYLE,
    opcodes::USER2_CREATE_RAIN_SCREEN,
    opcodes::USER2_RELEASE_RAIN_SCREEN,
    opcodes::USER2_INITIALIZE_RAIN_SCREEN,
    opcodes::USER2_SET_RAIN_TEXTURE,
    opcodes::USER2_SET_RAIN_ENABLED,
    opcodes::USER2_CONFIGURE_RAIN_DISPLAY,
    opcodes::USER2_SET_RAIN_VOLUME_BOUNDS,
    opcodes::USER2_SET_RAIN_DROP_WIDTH,
    opcodes::USER2_SET_RAIN_DROP_HEIGHT,
    opcodes::USER2_SET_RAIN_COLOR,
    opcodes::USER2_SET_RAIN_DENSITY,
    opcodes::USER2_SET_RAIN_SPEED,
    opcodes::USER2_SET_RAIN_ORIGIN,
    opcodes::USER2_SET_RAIN_DIRECTION,
    opcodes::USER2_SET_RAIN_LENGTH,
    opcodes::USER2_SET_RAIN_STEP,
    opcodes::GRAPH91_CONCENTRATE_BITMAP,
    opcodes::GRAPH91_SCALE_TRUE_COLOR_BITMAP,
    opcodes::GRAPH91_PROCESS_BITMAP,
    opcodes::GRAPH91_APPLY_GRAYSCALE_MASK,
    opcodes::GRAPH91_COMPOSITE_BITMAP_RECT_ALPHA,
    opcodes::GRAPH91_COMPOSITE_BITMAP_RECT_CONVERTED,
    opcodes::GRAPH90_APPLY_TONE_CURVE_EFFECT,
    opcodes::GRAPH90_SET_DISPLAY_OPTIONS,
    opcodes::GRAPH90_ENCODE_BITMAP_TO_BUFFER,
    opcodes::SYS_OPEN_FILE_DIALOG,
    opcodes::GRAPH90_UPLOAD_MAP_TILE_DATA,
    opcodes::SYS_LOAD_GLOBAL_USER_DATA,
    opcodes::SYS_SAVE_GLOBAL_USER_DATA,
    opcodes::SYS_WAIT_EXCLUSION_SECTION,
    opcodes::SYS_SET_VALIDATED_FILE_ROOT,
    opcodes::SYS_GET_CONFIGURED_ROOT,
    opcodes::SYS_SET_PRIMARY_ROOT,
    opcodes::SYS_WAIT_WINDOW_MESSAGE,
    opcodes::SYS_SET_MAIN_LOOP_WAIT_OVERRIDE,
    opcodes::SYS_SET_EXCLUSIVE_THREAD,
    opcodes::SYS_QUERY_FULLSCREEN,
    opcodes::SYS_CONFIGURE_FULLSCREEN_HOTKEYS,
    opcodes::SYS_SET_ASPECT_PRESERVING_SCALING,
    opcodes::SYS_SET_WINDOW_VISIBLE,
    opcodes::SYS_SET_WINDOW_TITLE,
    opcodes::SYS_SET_CURSOR_INDEX,
    opcodes::SYS_SET_NATIVE_CLOSE_MODE,
    opcodes::SYS_REQUEST_WINDOW_CLOSE,
    opcodes::SYS_TERMINATE_INTERPRETER,
    opcodes::SYS_SELECT_BOOTSTRAP,
    opcodes::SYS_SET_FILE_DROP_ENABLED,
    opcodes::SYS_SET_RASTER_WAIT_ENABLED,
    opcodes::SYS_QUERY_DISPLAY_ASPECT_MISMATCH,
    opcodes::SYS_SAVE_CONFIG_SLOT,
    opcodes::SYS_LOAD_CONFIG_SLOT,
    opcodes::SYS_READ_CONFIG_SLOT_HEADER,
    opcodes::SYS_VALIDATE_CONFIG_SLOT,
    opcodes::SYS_ENCODE_DATA,
    opcodes::SYS_ENCODE_STRUCT_ARRAY,
    opcodes::SYS_DECODE_DATA,
    opcodes::SYS_LAUNCH_PROCESS_WAIT,
    opcodes::SYS_RESTART_WITH_COMMAND,
    opcodes::SYS_LAUNCH_PROCESS_WAIT_UNINSTALLER,
    opcodes::SYS_SHELL_OPEN,
    opcodes::SOUND_SET_BGM_PRIMARY_VOLUME,
    opcodes::SOUND_SET_SE_PRIMARY_VOLUME,
    opcodes::SOUND_SET_BGM_SECONDARY_VOLUME,
    opcodes::SOUND_SET_SE_SECONDARY_VOLUME,
    opcodes::SOUND_QUERY_CD_MODE,
    opcodes::SYS_REQUIRE_RESOURCE_FILE,
    opcodes::SYS_CONFIGURE_REMOVABLE_ARCHIVE,
    opcodes::SYS_QUERY_LAUNCHER_MODE,
];

const PARTIAL_IMPLEMENTATION_OPCODES: &[NativeOpcode] = &[
    opcodes::SYS_GET_SPECIAL_FOLDER,
    opcodes::SYS81_SHOW_RESOURCE_FILE_DIALOG,
    opcodes::SYS81_BROWSE_FOLDER,
    opcodes::SYS81_SHOW_RESOURCE_LIST,
    opcodes::SYS81_RUN_INSTALLATION_PROCEDURE,
    opcodes::SYS81_VALIDATE_OR_CREATE_USER_PATH,
    opcodes::SYS_SHOW_INPUT_DIALOG,
    opcodes::SYS_SHOW_INSTALLER_DIALOG,
    opcodes::SYS_RUN_INSTALLER_WORKFLOW,
    opcodes::SYS_RUN_SHORTCUT_INSTALLER_WORKFLOW,
    opcodes::SYS_CREATE_SHORTCUT,
    opcodes::SYS_READ_INSTALLED_FOLDER,
    opcodes::SYS_DELETE_INSTALLED_REGISTRY_KEY,
    opcodes::SYS_REGISTER_FILE_ASSOCIATION,
    opcodes::SYS_WRITE_WINDOWS_DIRECTORY,
    opcodes::GRAPH90_IMPORT_EXTERNAL_IMAGE,
    opcodes::GRAPH90_SAVE_BITMAP_TO_IMAGE_FILE,
    opcodes::SYS81_CREATE_CHILD_THREAD,
    opcodes::SYS81_ENUMERATE_RESOURCES,
    opcodes::SYS81_QUEUE_RESOURCE_SEEK,
    opcodes::SYS81_QUEUE_RESOURCE_READ,
    opcodes::SYS81_QUEUE_RESOURCE_CLOSE,
    opcodes::SYS81_GET_FILE_TIMES,
    opcodes::SYS81_SET_FILE_TIMES,
    opcodes::SYS81_INTERNET_READ,
    opcodes::SYS81_ENUMERATE_DRIVE_TYPES,
    opcodes::SYS81_GET_DISK_FREE_MB,
    opcodes::SYS81_GET_VOLUME_LABEL,
    opcodes::SYS81_QUERY_DEVICE_POWER_STATE,
    opcodes::SYS_CONFIGURE_DISPLAY_MODE,
    opcodes::SYS_REMOVE_INSTALLER_SHORTCUTS,
    opcodes::SYS_READ_WINDOWS_PATH_FILE,
    opcodes::GRAPH92_PRELOAD_BITMAP_RESOURCE,
    opcodes::GRAPH92_CANCEL_PENDING_BITMAP_PRELOADS,
    opcodes::GRAPH92_LOAD_EXTERNAL_BMP,
    opcodes::GRAPH92_LOAD_BURIKO_MOVIE_RESOURCE,
    opcodes::SYS81_OPEN_RESOURCE_STREAM,
    opcodes::SYS81_TEST_PATH_WRITABLE,
    opcodes::SYS81_READ_RESOURCE_BINARY,
    opcodes::SYS81_DIRECT_FILE_READ,
    opcodes::SYS81_RESOURCE_SIZE,
    opcodes::SYS81_RESOURCE_EXISTS,
    opcodes::SYS81_SET_ERROR_CAPTURE_MODE,
    opcodes::SYS81_COPY_CAPTURED_ERROR,
    opcodes::SYS_ALLOC,
    opcodes::SYS_FREE,
    opcodes::SYS_ALLOCATE_GLOBAL_CONFIG,
    opcodes::SYS_CLEAR_GLOBAL_CONFIG,
    opcodes::SYS_REMOVE_UNINSTALL_LISTED_FILES,
    opcodes::SYS_APPEND_UNINSTALL_LIST_ENTRIES,
    opcodes::SYS_LOAD_PROGRAM_MODULE,
    opcodes::SYS_FREE_LAST_PROGRAM_MODULE,
    opcodes::SYS_LOAD_PROGRAM_THREAD,
];

const COMPATIBILITY_FALLBACK_OPCODES: &[NativeOpcode] = &[
];

const STUB_OPCODES: &[NativeOpcode] = &[
];

pub fn recovery_level(opcode: NativeOpcode) -> NativeRecoveryLevel {
    if TARGET_CONFIRMED_OPCODES.contains(&opcode) {
        NativeRecoveryLevel::TargetConfirmed
    } else if TARGET_INFERRED_OPCODES.contains(&opcode) {
        NativeRecoveryLevel::TargetInferred
    } else if ABI_ONLY_DOCUMENTED_OPCODES.contains(&opcode) || documented_opcode(opcode).is_some() {
        NativeRecoveryLevel::AbiOnly
    } else if matches!(
        candidate_call_name(opcode.group, opcode.id),
        Some(name) if name != "UnrecoveredNativeCall"
    ) {
        NativeRecoveryLevel::CandidateNameOnly
    } else {
        NativeRecoveryLevel::Unrecovered
    }
}

pub fn display_name(opcode: NativeOpcode) -> &'static str {
    documented_opcode(opcode)
        .map(|spec| spec.symbol)
        .or_else(|| candidate_call_name(opcode.group, opcode.id))
        .unwrap_or("UnknownNativeCall")
}

pub fn is_strictly_supported(opcode: NativeOpcode) -> bool {
    matches!(
        implementation_level(opcode),
        NativeImplementationLevel::PortableEquivalent
            | NativeImplementationLevel::Partial
            | NativeImplementationLevel::CompatibilityFallback
    )
}

pub fn implementation_level(opcode: NativeOpcode) -> NativeImplementationLevel {
    if PORTABLE_EQUIVALENT_OPCODES.contains(&opcode) {
        NativeImplementationLevel::PortableEquivalent
    } else if PARTIAL_IMPLEMENTATION_OPCODES.contains(&opcode) {
        NativeImplementationLevel::Partial
    } else if COMPATIBILITY_FALLBACK_OPCODES.contains(&opcode) {
        NativeImplementationLevel::CompatibilityFallback
    } else if STUB_OPCODES.contains(&opcode) {
        NativeImplementationLevel::Stub
    } else {
        NativeImplementationLevel::NotAudited
    }
}

/// Return detailed documentation when parameter names and scheduling semantics
/// have been explicitly recovered.
pub fn documented_opcode(opcode: NativeOpcode) -> Option<&'static NativeOpcodeSpec> {
    DOCUMENTED_OPCODES.iter().find(|spec| spec.opcode == opcode)
}

/// Return the scheduling effect known from either an explicit opcode wrapper or
/// the recovered native ABI procedure table.
pub fn scheduling_effect(opcode: NativeOpcode) -> NativeSchedulingEffect {
    documented_opcode(opcode)
        .map(|spec| spec.scheduling)
        .unwrap_or_else(|| {
            if native_abi::installs_procedure(opcode.group, opcode.id) {
                NativeSchedulingEffect::WaitProcedure
            } else {
                NativeSchedulingEffect::Continue
            }
        })
}

/// A validated native call frame.
///
/// Arguments are stored in BP call order. `pop_*` consumes them from the end,
/// matching the native handlers recovered from the original stack machine.
/// Keeping the opcode and its arguments together prevents handlers from being
/// called with an unrelated `(group, id)` pair and an anonymous vector.
#[derive(Debug, Clone)]
pub struct NativeCallFrame {
    opcode: NativeOpcode,
    args: Vec<Value>,
    procedure_completion: Option<NativeProcedureCompletion>,
    procedure_start: Option<NativeProcedureStart>,
}

/// Host-side completion information for a native handler that the target
/// represents as a cooperative `CProcedure`. This is not an immediate BP
/// return value: the VM installs the recovered procedure class, polls its
/// terminal state, destroys it, and only then resumes the script.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativeProcedureCompletion {
    pub class: NativeProcedureClass,
    pub status: i32,
    /// Values produced by the procedure when its terminal virtual poll runs.
    /// They are deliberately separate from the immediate native return path.
    pub outputs: [i32; 2],
    pub output_count: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeProcedureClass {
    LoadSound,
    RegisterSound,
    EncodeData,
    EncodeStruct,
    DecodeData,
    Exclusion,
    Load,
    ReadBinary,
    LoadBitmap,
    PreloadBitmap,
    LoadBurikoMovie,
    LoadBurikoMovieHeader,
    DecodeBurikoMovie,
    Installation,
}

impl NativeProcedureClass {
    pub const fn target_class_name(self) -> &'static str {
        match self {
            Self::LoadSound => "CProcLoadSound",
            Self::RegisterSound => "DCProcRgstrSound",
            Self::EncodeData => "CProcEncodeData",
            Self::EncodeStruct => "CProcEncodeStruct",
            Self::DecodeData => "DCProcDecodeData",
            Self::Exclusion => "CProcExclusion",
            Self::Load => "CProcLoad",
            Self::ReadBinary => "DCProcReadBinary",
            Self::LoadBitmap => "CProcLoadBitmap",
            Self::PreloadBitmap => "DCProcPreloadBmp",
            Self::LoadBurikoMovie => "DCProcLoadBurikoMV",
            Self::LoadBurikoMovieHeader => "DCProcLoadBMVHeader",
            Self::DecodeBurikoMovie => "DCProcDecodeBMV",
            Self::Installation => "DCProcInstallation",
        }
    }
}

/// Concrete target message-procedure class selected by the graph dispatch
/// allocation path.  These names come from the target RTTI/vtable catalog,
/// not from third-party handler labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeMessageProcedureClass {
    DspMsg,
    DspMsgEx,
    DspMsgExVE,
}

/// Portable call contract for the recovered `CProcDspMsg` hierarchy.
///
/// The four trailing control arguments are mapped from the exact wrapper pop
/// order into target members `+0x30`, `+0x38`, `+0x3c`, and `+0x40`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativeMessageProcedureConfig {
    pub class: NativeMessageProcedureClass,
    pub initial_delay_enabled: bool,
    pub initial_delay_ms: i32,
    pub reveal_duration_ms: i32,
    pub reveal_steps: i32,
    pub reveal_step_delay_ms: i32,
    pub settle_steps: i32,
    pub settle_step_delay_ms: i32,
    pub auto_advance_delay_ms: Option<i32>,
    /// Exact CProcDspMsg+0x78 scope representation. Mode 0 stores literal
    /// `2`; modes 1/2 already store `(scope << 16) | 0xFFFF`. The procedure
    /// constructor/destructor register and remove this value without repacking.
    pub input_scope: i32,
    pub completion_control: i32,
    pub end_wait_policy: i32,
    pub allow_high_bit_input: bool,
    pub allow_auxiliary_input: bool,
    pub auxiliary_input_mask: i32,
    pub input_forces_completion: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeProcedureStart {
    Message(NativeMessageProcedureConfig),
    /// Object-specific CProcCtrlDspObj installation.  The target procedure
    /// owns one CDspObj; waiting on a selector-global "any animation" flag
    /// lets unrelated animations stall or prematurely release this CThread.
    GraphControl {
        object_id: i32,
        control_id: u64,
    },
}

impl NativeCallFrame {
    pub fn new(opcode: NativeOpcode, args: Vec<Value>) -> Self {
        Self {
            opcode,
            args,
            procedure_completion: None,
            procedure_start: None,
        }
    }

    pub fn opcode(&self) -> NativeOpcode {
        self.opcode
    }

    pub fn group(&self) -> u8 {
        self.opcode.group
    }

    pub fn id(&self) -> u16 {
        self.opcode.id
    }

    pub fn name(&self) -> &'static str {
        display_name(self.opcode)
    }

    pub fn recovery_level(&self) -> NativeRecoveryLevel {
        recovery_level(self.opcode)
    }

    pub fn implementation_level(&self) -> NativeImplementationLevel {
        implementation_level(self.opcode)
    }

    pub fn documented_spec(&self) -> Option<&'static NativeOpcodeSpec> {
        documented_opcode(self.opcode)
    }

    pub fn scheduling_effect(&self) -> NativeSchedulingEffect {
        scheduling_effect(self.opcode)
    }

    pub fn args(&self) -> &[Value] {
        &self.args
    }

    /// Transitional access for legacy handlers. New or reverse-engineered
    /// handlers should use the named `pop_*` methods below.
    pub fn args_mut(&mut self) -> &mut Vec<Value> {
        &mut self.args
    }

    pub fn remaining(&self) -> usize {
        self.args.len()
    }

    /// Report that a target `CProcedure`-installing handler has reached a
    /// terminal host state. The value is consumed by the VM scheduler and is
    /// never exposed as an immediate native return.
    pub fn complete_procedure(&mut self, class: NativeProcedureClass, status: i32) {
        self.procedure_completion = Some(NativeProcedureCompletion {
            class,
            status,
            outputs: [0; 2],
            output_count: 0,
        });
    }

    /// Complete a target procedure with one deferred BP stack result.
    pub fn complete_procedure_with_output(
        &mut self,
        class: NativeProcedureClass,
        status: i32,
        output: i32,
    ) {
        self.procedure_completion = Some(NativeProcedureCompletion {
            class,
            status,
            outputs: [output, 0],
            output_count: 1,
        });
    }

    /// Complete a target procedure with two deferred BP stack results in push order.
    pub fn complete_procedure_with_outputs(
        &mut self,
        class: NativeProcedureClass,
        status: i32,
        outputs: [i32; 2],
    ) {
        self.procedure_completion = Some(NativeProcedureCompletion {
            class,
            status,
            outputs,
            output_count: 2,
        });
    }

    pub(crate) fn take_procedure_completion(&mut self) -> Option<NativeProcedureCompletion> {
        self.procedure_completion.take()
    }

    /// Install a target message-display procedure after the graph handler has
    /// created the text state.  This is a scheduler boundary, not an immediate
    /// return value.
    pub fn start_message_procedure(&mut self, config: NativeMessageProcedureConfig) {
        self.procedure_start = Some(NativeProcedureStart::Message(config));
    }

    /// Install the target CProcCtrlDspObj for the explicit object consumed by
    /// this graph call.  This must be attached to the call frame so the VM can
    /// bind CThread+0x58 to the same object instead of waiting on every active
    /// graph animation in the process.
    pub fn start_graph_control_procedure(&mut self, object_id: i32, control_id: u64) {
        self.procedure_start = Some(NativeProcedureStart::GraphControl {
            object_id,
            control_id,
        });
    }

    pub(crate) fn take_procedure_start(&mut self) -> Option<NativeProcedureStart> {
        self.procedure_start.take()
    }

    /// Verify that a typed native handler consumed every input parameter.
    /// This is intentionally strict for recovered wrappers: silently leaving
    /// values behind usually means the reverse-engineered signature is wrong.
    pub fn require_consumed(&self) -> VmResult<()> {
        if self.args.is_empty() {
            return Ok(());
        }
        Err(VmError::Runtime(format!(
            "{} (0x{:02X}:0x{:02X}) left {} unconsumed parameters",
            self.name(),
            self.group(),
            self.id(),
            self.args.len()
        )))
    }

    pub fn into_args(self) -> Vec<Value> {
        self.args
    }

    pub fn pop_value(&mut self, parameter: &'static str) -> VmResult<Value> {
        self.args.pop().ok_or_else(|| {
            VmError::Runtime(format!(
                "{} (0x{:02X}:0x{:02X}) missing parameter `{parameter}`",
                self.name(),
                self.group(),
                self.id()
            ))
        })
    }

    pub fn pop_i32(&mut self, parameter: &'static str) -> VmResult<i32> {
        Ok(self.pop_value(parameter)?.as_i32())
    }

    pub fn pop_ptr(&mut self, parameter: &'static str) -> VmResult<u32> {
        match self.pop_value(parameter)? {
            Value::Ptr(value) => Ok(value),
            Value::Int(value) => Ok(value as u32),
            value => Err(VmError::Runtime(format!(
                "{} (0x{:02X}:0x{:02X}) parameter `{parameter}` expected pointer, got {value:?}",
                self.name(),
                self.group(),
                self.id()
            ))),
        }
    }

    pub fn pop_string(&mut self, parameter: &'static str) -> VmResult<String> {
        match self.pop_value(parameter)? {
            Value::Str(value) => Ok(value),
            Value::Int(0) | Value::Ptr(0) | Value::None => Ok(String::new()),
            value => Err(VmError::Runtime(format!(
                "{} (0x{:02X}:0x{:02X}) parameter `{parameter}` expected string, got {value:?}",
                self.name(),
                self.group(),
                self.id()
            ))),
        }
    }

    /// Out parameters in several BGI ABIs are returned through the call frame.
    pub fn push_output(&mut self, value: Value) {
        self.args.push(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scheduler_opcodes_have_named_parameter_documentation() {
        let yield_spec = documented_opcode(opcodes::SYS_YIELD).expect("yield documentation");
        assert_eq!(yield_spec.symbol, "Sys80_5F_Yield");
        assert_eq!(yield_spec.parameters, NO_PARAMETERS);
        assert_eq!(yield_spec.scheduling, NativeSchedulingEffect::Yield);

        let event_spec = documented_opcode(opcodes::SYS_QUERY_INPUT_EVENT_BITS)
            .expect("input event documentation");
        assert_eq!(event_spec.symbol, "Sys80_1A_QueryInputEventBits");
        assert_eq!(event_spec.parameters[0].name, "scope");
        assert_eq!(event_spec.scheduling, NativeSchedulingEffect::Continue);

        let wait_spec = documented_opcode(opcodes::SYS_WAIT_TIMING_EX).expect("wait documentation");
        assert_eq!(wait_spec.parameters.len(), 3);
        assert_eq!(wait_spec.parameters[0].name, "duration_ms");
    }

    #[test]
    fn audited_symbol_overrides_legacy_candidate_name() {
        let frame = NativeCallFrame::new(
            opcodes::GRAPH_SET_PERSISTENT_TEXT_STYLE,
            vec![Value::Int(0); 7],
        );
        assert_eq!(frame.name(), "Graph92_97_SetPersistentTextStyleFields");
        assert_eq!(frame.recovery_level(), NativeRecoveryLevel::TargetConfirmed);
        assert_eq!(
            frame.implementation_level(),
            NativeImplementationLevel::PortableEquivalent
        );
    }

    #[test]
    fn frame_preserves_native_stack_pop_order() {
        let mut frame = NativeCallFrame::new(
            opcodes::SYS_WAIT_TIMING_EX,
            vec![Value::Int(30), Value::Int(1), Value::Int(7)],
        );
        assert_eq!(frame.pop_i32("input_scope").unwrap(), 7);
        assert_eq!(frame.pop_i32("input_enabled").unwrap(), 1);
        assert_eq!(frame.pop_i32("duration_ms").unwrap(), 30);
        assert_eq!(frame.remaining(), 0);
    }

    #[test]
    fn typed_wrappers_have_explicit_recovery_and_implementation_status() {
        for spec in DOCUMENTED_OPCODES {
            assert!(!matches!(
                recovery_level(spec.opcode),
                NativeRecoveryLevel::CandidateNameOnly | NativeRecoveryLevel::Unrecovered
            ));
            assert_ne!(
                implementation_level(spec.opcode),
                NativeImplementationLevel::NotAudited
            );
        }
    }

    #[test]
    fn recovered_selectors_follow_the_current_target_evidence() {
        let transform = NativeOpcode {
            group: 0x90,
            id: 0x5c,
        };
        assert_eq!(
            recovery_level(transform),
            NativeRecoveryLevel::TargetConfirmed
        );
        assert_eq!(
            implementation_level(transform),
            NativeImplementationLevel::PortableEquivalent
        );

        let clear_config = NativeOpcode {
            group: 0x80,
            id: 0x71,
        };
        assert_eq!(
            recovery_level(clear_config),
            NativeRecoveryLevel::TargetConfirmed
        );
        // Deferred on the VM value model (ADR 0003).
        assert_eq!(
            implementation_level(clear_config),
            NativeImplementationLevel::Partial
        );

        let unknown = NativeOpcode {
            group: 0x7f,
            id: 0xff,
        };
        assert_eq!(recovery_level(unknown), NativeRecoveryLevel::Unrecovered);
    }

    #[test]
    fn recovered_host_procedure_classes_keep_target_rtti_names() {
        let expected = [
            (NativeProcedureClass::LoadSound, "CProcLoadSound"),
            (NativeProcedureClass::RegisterSound, "DCProcRgstrSound"),
            (NativeProcedureClass::EncodeData, "CProcEncodeData"),
            (NativeProcedureClass::EncodeStruct, "CProcEncodeStruct"),
            (NativeProcedureClass::DecodeData, "DCProcDecodeData"),
            (NativeProcedureClass::Exclusion, "CProcExclusion"),
            (NativeProcedureClass::Load, "CProcLoad"),
            (NativeProcedureClass::ReadBinary, "DCProcReadBinary"),
            (NativeProcedureClass::LoadBitmap, "CProcLoadBitmap"),
            (NativeProcedureClass::PreloadBitmap, "DCProcPreloadBmp"),
            (NativeProcedureClass::LoadBurikoMovie, "DCProcLoadBurikoMV"),
            (
                NativeProcedureClass::LoadBurikoMovieHeader,
                "DCProcLoadBMVHeader",
            ),
            (NativeProcedureClass::DecodeBurikoMovie, "DCProcDecodeBMV"),
            (NativeProcedureClass::Installation, "DCProcInstallation"),
        ];
        for (class, name) in expected {
            assert_eq!(class.target_class_name(), name);
        }
    }

    #[test]
    fn target_bitmap_wrappers_document_bp_argument_direction() {
        let alpha =
            documented_opcode(opcodes::GRAPH_SET_OBJECT_ALPHA).expect("Graph90:32 documentation");
        assert_eq!(alpha.parameters[0].name, "object");
        assert_eq!(alpha.parameters[1].name, "alpha_parameter");

        let transition = documented_opcode(opcodes::GRAPH90_CONFIGURE_SPRITE_DUAL_BITMAP)
            .expect("Graph90:58 documentation");
        assert_eq!(transition.parameters[0].name, "sprite");
        assert_eq!(transition.parameters[3].name, "primary_bitmap");
        assert_eq!(transition.parameters[8].name, "transition_mode");

        let release_filter = documented_opcode(opcodes::GRAPH90_RELEASE_FILTER_OBJECT)
            .expect("Graph90:61 documentation");
        assert_eq!(release_filter.parameters[0].name, "filter");

        let render = documented_opcode(opcodes::GRAPH_RENDER_OBJECT_TO_BITMAP)
            .expect("Graph90:83 documentation");
        assert_eq!(render.parameters[0].name, "destination_bitmap");
        assert_eq!(render.parameters[1].name, "source_window");

        let bind = documented_opcode(opcodes::GRAPH_BIND_BITMAP_TO_SURFACE)
            .expect("Graph90:86 documentation");
        assert_eq!(bind.parameters[0].name, "window");
        assert_eq!(bind.parameters[3].name, "source_bitmap");

        let convert = documented_opcode(opcodes::GRAPH_CONVERT_BITMAP_TO_ALPHA_DESCRIPTOR)
            .expect("Graph92:18 documentation");
        assert_eq!(convert.parameters[0].name, "destination_descriptor");
        assert_eq!(convert.parameters[1].name, "source_descriptor");

        let extent = documented_opcode(opcodes::GRAPH_SET_TEXT_EXTENT_FONT_STATE)
            .expect("Graph91:98 documentation");
        assert_eq!(extent.parameters.len(), 6);
        assert_eq!(extent.parameters[0].name, "layout_advance");
        assert_eq!(extent.parameters[5].name, "mode_flag");

        let style = documented_opcode(opcodes::GRAPH_SET_PERSISTENT_TEXT_STYLE)
            .expect("Graph92:97 documentation");
        assert_eq!(style.parameters.len(), 7);
        assert_eq!(style.parameters[0].name, "font_name");
        assert_eq!(style.parameters[1].name, "ruby_height");
        assert_eq!(style.parameters[3].name, "ruby_x_offset");
        assert_eq!(style.parameters[4].name, "ruby_y_offset");
        assert_eq!(style.parameters[6].name, "text_override_1");
    }
}
