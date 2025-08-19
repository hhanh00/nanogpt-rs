import 'dart:io';

import 'package:flutter/material.dart';
import 'package:logger/logger.dart';
import 'package:nanogpt/src/rust/api/simple.dart';
import 'package:nanogpt/src/rust/frb_generated.dart';
import 'package:nanogpt/store.dart';
import 'package:path_provider/path_provider.dart';
import 'package:path/path.dart' as p;

var logger = Logger();

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text('flutter_rust_bridge quickstart'),
          actions: [
            IconButton(
              onPressed: onDownloadTrainingData,
              icon: Icon(Icons.download),
              tooltip: "Download training data",
            ),
            IconButton(onPressed: appStore.inc, icon: Icon(Icons.plus_one)),
          ],
        ),
        body: SizedBox.shrink(),
      ),
    );
  }

  void onDownloadTrainingData() async {
    final path = await getApplicationDocumentsDirectory();
    final trainingFile = File(p.join(path.path, "training.txt"));
    await trainingFile.create();
    await downloadTrainingData(
      path: trainingFile.path,
      url:
          "https://raw.githubusercontent.com/karpathy/char-rnn/master/data/tinyshakespeare/input.txt",
    );
    logger.i("Training data downloaded to ${trainingFile.path}");
  }
}
