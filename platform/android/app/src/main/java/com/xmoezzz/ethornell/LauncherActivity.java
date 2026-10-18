package com.xmoezzz.ethornell;

import android.app.Activity;
import android.app.AlertDialog;
import android.content.Intent;
import android.net.Uri;
import android.os.Bundle;
import android.os.Handler;
import android.os.Looper;
import android.widget.ArrayAdapter;
import android.widget.Button;
import android.widget.ListView;
import android.widget.TextView;

import java.util.List;

/** The game list: import a folder, pick a game, set its text encoding. */
public final class LauncherActivity extends Activity {
    private static final int REQUEST_IMPORT = 1;

    private GameLibrary library;
    private ArrayAdapter<String> adapter;
    private TextView empty;
    private final Handler ui = new Handler(Looper.getMainLooper());

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_launcher);
        library = new GameLibrary(this);

        TextView hint = findViewById(R.id.hint);
        hint.setText(getString(R.string.games_folder_hint, library.gamesDir().getAbsolutePath()));
        empty = findViewById(R.id.empty);

        Button importButton = findViewById(R.id.import_button);
        importButton.setOnClickListener(v -> {
            Intent intent = new Intent(Intent.ACTION_OPEN_DOCUMENT_TREE);
            startActivityForResult(intent, REQUEST_IMPORT);
        });

        adapter = new ArrayAdapter<>(this, android.R.layout.simple_list_item_1);
        ListView list = findViewById(R.id.games);
        list.setAdapter(adapter);
        list.setOnItemClickListener((parent, view, position, id) -> play(adapter.getItem(position)));
        list.setOnItemLongClickListener((parent, view, position, id) -> {
            options(adapter.getItem(position));
            return true;
        });
    }

    @Override
    protected void onResume() {
        super.onResume();
        refresh();
    }

    private void refresh() {
        List<String> games = library.games();
        adapter.clear();
        adapter.addAll(games);
        empty.setVisibility(games.isEmpty() ? TextView.VISIBLE : TextView.GONE);
    }

    private void play(String game) {
        Intent intent = new Intent(this, GameActivity.class);
        intent.putExtra(GameActivity.EXTRA_GAME, game);
        startActivity(intent);
    }

    private void options(String game) {
        String[] encodings = {getString(R.string.encoding_sjis), getString(R.string.encoding_gbk)};
        new AlertDialog.Builder(this)
                .setTitle(game)
                .setSingleChoiceItems(encodings, library.textEncoding(game), (dialog, which) -> {
                    library.setTextEncoding(game, which);
                    dialog.dismiss();
                })
                .setNegativeButton(R.string.delete, (dialog, which) -> confirmDelete(game))
                .setPositiveButton(android.R.string.cancel, null)
                .show();
    }

    private void confirmDelete(String game) {
        new AlertDialog.Builder(this)
                .setMessage(getString(R.string.delete_confirm, game))
                .setPositiveButton(R.string.delete, (dialog, which) -> {
                    library.delete(game);
                    refresh();
                })
                .setNegativeButton(android.R.string.cancel, null)
                .show();
    }

    @Override
    protected void onActivityResult(int requestCode, int resultCode, Intent data) {
        super.onActivityResult(requestCode, resultCode, data);
        if (requestCode != REQUEST_IMPORT || resultCode != RESULT_OK || data == null) {
            return;
        }
        Uri tree = data.getData();
        if (tree == null) {
            return;
        }
        AlertDialog progress = new AlertDialog.Builder(this)
                .setTitle(R.string.importing)
                .setMessage("")
                .setCancelable(false)
                .show();
        new Thread(() -> {
            String error = null;
            try {
                library.importTree(tree, name -> ui.post(() -> progress.setMessage(name)));
            } catch (Exception e) {
                error = e.getMessage();
            }
            String failure = error;
            ui.post(() -> {
                progress.dismiss();
                refresh();
                if (failure != null) {
                    new AlertDialog.Builder(this)
                            .setTitle(R.string.import_failed)
                            .setMessage(failure)
                            .setPositiveButton(android.R.string.ok, null)
                            .show();
                }
            });
        }, "import").start();
    }
}
