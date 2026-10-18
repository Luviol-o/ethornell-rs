package com.xmoezzz.ethornell;

import android.content.Context;
import android.view.Surface;

/**
 * The engine in libethornell.so (crates/ethornell-mobile). Every call except
 * {@link #nativeInit} and {@link #nativeLastError} runs on the one engine
 * thread that created the handle (platform/android/include/ABI.md).
 */
final class NativeEngine {
    static {
        System.loadLibrary("ethornell");
    }

    private NativeEngine() {}

    // Input kinds of EthornellInputV1.
    static final int INPUT_MOVE = 1;
    static final int INPUT_BUTTON_DOWN = 2;
    static final int INPUT_BUTTON_UP = 3;
    static final int INPUT_KEY_DOWN = 4;
    static final int INPUT_KEY_UP = 5;
    static final int INPUT_WHEEL = 6;
    static final int INPUT_CANCEL = 7;

    // Windows button descriptors.
    static final int BUTTON_LEFT = 1;
    static final int BUTTON_RIGHT = 2;

    // Event kinds of EthornellEventV1.
    static final int EVENT_DIAGNOSTIC = 1;
    static final int EVENT_CONFIRMATION = 2;
    static final int EVENT_CLOSED = 3;
    static final int EVENT_TITLE = 4;

    // Pause reasons.
    static final int PAUSE_ACTIVITY = 1;

    static final int STATUS_OK = 0;
    static final int STATUS_NOT_YET = 1;

    static native int nativeInit(Context application);

    /** Returns the handle, or 0 on failure (see {@link #nativeLastError}). */
    static native long nativeCreate(
            String resourceRoot, String userRoot, String fontPath, int textEncoding, int flags);

    static native int nativeSetSurface(
            long handle, Surface surface, int widthPx, int heightPx, long generation);

    static native int nativeDetachSurface(long handle, long generation);

    static native int nativeStep(long handle, long monotonicTimeNs);

    static native int nativeInput(long handle, int kind, float x, float y, int code, float value);

    static native int nativeSetPaused(long handle, int reasonMask);

    static native int nativeRequestClose(long handle);

    /** The next event's text, or null; meta receives kind, request id, code. */
    static native String nativePollEvent(long handle, long[] meta);

    static native int nativeCompleteRequest(long handle, long requestId, int result);

    static native int nativeDestroy(long handle);

    static native String nativeLastError();
}
