import 'package:flutter/material.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:nanogpt/src/rust/api/simple.dart';
import 'package:nanogpt/src/rust/frb_generated.dart';
import 'package:nanogpt/store.dart';

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
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart'),
        actions: [
          IconButton(onPressed: appStore.inc, icon: Icon(Icons.plus_one))
        ]),
        body: Center(
          child: Observer(builder: (context) => Text(
            'Action: Call Rust `greet("Tom")`\nResult: `${greet(name: "Tom")} ${appStore.value} times`',
          ),
        ),
      ),
    ));
  }
}
