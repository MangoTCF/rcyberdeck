import 'package:cyberreck/messages/basic.pb.dart';
import 'package:flutter/widgets.dart';
import 'package:rinf/rinf.dart';
import 'package:cyberreck/messages/generated.dart';

class BootWidget extends StatefulWidget {
  const BootWidget({super.key});
  @override
  State<StatefulWidget> createState() => _BootWidgetState();
}

class _BootWidgetState extends State<BootWidget> {
  List fullLog = [];
  int logIndex = 0;
  String log = "Waiting for rBIOS v3.01 init...\n";

  void updateLog() {
    var line = (fullLog[logIndex] as String);
    var parts = line.split(' ');
    var millis = int.tryParse(parts[0].trim()) ?? 100;
    if (millis == -1) {
      return;
    }
    var logEntry = "${parts.sublist(1).join(' ').trim()}\n";
    setState(() {
      log += logEntry;
    });

    Future.delayed(Duration(milliseconds: millis), () {
      updateLog();
    });
    logIndex++;
  }

  @override
  Widget build(BuildContext context) {
    DefaultAssetBundle.of(context)
        .loadString("assets/misc/bootlog.txt")
        .then((assetLog) {
      fullLog = assetLog.split("\n");
      Future.delayed(const Duration(milliseconds: 100), () {
        updateLog();
      });
    });
    return Text(log);
  }
}
