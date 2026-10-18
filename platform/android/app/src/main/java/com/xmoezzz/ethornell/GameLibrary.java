package com.xmoezzz.ethornell;

import android.content.Context;
import android.content.SharedPreferences;
import android.net.Uri;

import androidx.documentfile.provider.DocumentFile;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Locale;

/**
 * Games live in the app's external files directory
 * (Android/data/com.xmoezzz.ethornell/files/games/&lt;name&gt;), one folder each,
 * either imported through the folder picker or copied there over USB. Saves
 * go to the app's private storage, apart from the game files.
 */
final class GameLibrary {
    static final int ENCODING_SHIFT_JIS = 0;
    static final int ENCODING_GBK = 1;

    private final Context context;
    private final SharedPreferences prefs;

    GameLibrary(Context context) {
        this.context = context.getApplicationContext();
        this.prefs = this.context.getSharedPreferences("games", Context.MODE_PRIVATE);
    }

    File gamesDir() {
        File base = context.getExternalFilesDir(null);
        if (base == null) {
            base = context.getFilesDir();
        }
        File dir = new File(base, "games");
        //noinspection ResultOfMethodCallIgnored
        dir.mkdirs();
        return dir;
    }

    File userDir(String game) {
        File dir = new File(new File(context.getFilesDir(), "userdata"), game);
        //noinspection ResultOfMethodCallIgnored
        dir.mkdirs();
        return dir;
    }

    /** The folders under games/ that hold BGI archives (*.arc). */
    List<String> games() {
        List<String> names = new ArrayList<>();
        File[] dirs = gamesDir().listFiles(File::isDirectory);
        if (dirs != null) {
            for (File dir : dirs) {
                if (isGame(dir)) {
                    names.add(dir.getName());
                }
            }
        }
        names.sort(String.CASE_INSENSITIVE_ORDER);
        return names;
    }

    static boolean isGame(File dir) {
        String[] files = dir.list();
        return files != null
                && Arrays.stream(files)
                        .anyMatch(name -> name.toLowerCase(Locale.ROOT).endsWith(".arc"));
    }

    File gameDir(String game) {
        return new File(gamesDir(), game);
    }

    int textEncoding(String game) {
        return prefs.getInt("encoding:" + game, ENCODING_SHIFT_JIS);
    }

    void setTextEncoding(String game, int encoding) {
        prefs.edit().putInt("encoding:" + game, encoding).apply();
    }

    void delete(String game) {
        deleteTree(gameDir(game));
        prefs.edit().remove("encoding:" + game).apply();
    }

    interface Progress {
        void copied(String name);
    }

    /** Copies a picked folder into games/; returns the new game's name. */
    String importTree(Uri treeUri, Progress progress) throws IOException {
        DocumentFile tree = DocumentFile.fromTreeUri(context, treeUri);
        if (tree == null || !tree.isDirectory()) {
            throw new IOException("not a folder");
        }
        String name = tree.getName() != null ? tree.getName() : "game";
        File target = new File(gamesDir(), name);
        for (int n = 2; target.exists(); n++) {
            target = new File(gamesDir(), name + " (" + n + ")");
        }
        File staging = new File(gamesDir(), ".import-" + System.nanoTime());
        try {
            copyTree(tree, staging, progress);
            if (!isGame(staging)) {
                throw new IOException("no .arc archives in the folder");
            }
            if (!staging.renameTo(target)) {
                throw new IOException("cannot create " + target);
            }
        } finally {
            deleteTree(staging);
        }
        return target.getName();
    }

    private void copyTree(DocumentFile src, File dst, Progress progress) throws IOException {
        if (!dst.mkdirs() && !dst.isDirectory()) {
            throw new IOException("cannot create " + dst);
        }
        for (DocumentFile child : src.listFiles()) {
            String name = child.getName();
            if (name == null || name.equals(".") || name.equals("..")) {
                continue;
            }
            File out = new File(dst, name);
            if (child.isDirectory()) {
                copyTree(child, out, progress);
            } else if (child.isFile()) {
                progress.copied(name);
                try (InputStream in = context.getContentResolver().openInputStream(child.getUri());
                        OutputStream os = new FileOutputStream(out)) {
                    if (in == null) {
                        throw new IOException("cannot read " + name);
                    }
                    byte[] buffer = new byte[1 << 16];
                    for (int n; (n = in.read(buffer)) > 0; ) {
                        os.write(buffer, 0, n);
                    }
                }
            }
        }
    }

    private static void deleteTree(File file) {
        File[] children = file.listFiles();
        if (children != null) {
            for (File child : children) {
                deleteTree(child);
            }
        }
        //noinspection ResultOfMethodCallIgnored
        file.delete();
    }
}
