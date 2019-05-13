import 'dart:io';

import 'package:camera/camera.dart';
import 'package:flutter/material.dart';
import 'package:permission/permission.dart';
import 'package:register_app/pages/HomePage.dart';
import 'package:register_app/pages/LandingPage.dart';
import 'package:register_app/pages/RegisterPage.dart';

List<CameraDescription> cameras;

Future<void> main() async {
  cameras = await availableCameras();
  runApp(MyApp());
}

bool isPositivePermissionStatus(PermissionStatus status) {
  if (status == null) return false;

  if (status == PermissionStatus.allow ||
      status == PermissionStatus.always ||
      status == PermissionStatus.whenInUse ||
      status == PermissionStatus.notDecided) {
    return true;
  }

  return false;
}

class MyApp extends StatefulWidget {
  @override
  State<StatefulWidget> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  bool _loading = true;
  PermissionStatus _cameraPermissionStatus;
  PermissionStatus _microphonePermissionStatus;

  bool get _hasRequiredPermissions =>
      isPositivePermissionStatus(_cameraPermissionStatus) &&
      isPositivePermissionStatus(_microphonePermissionStatus);

  @override
  void initState() {
    super.initState();

    initialize();
  }

  void initialize() async {
    if (Platform.isAndroid) {
      final permissionList = [PermissionName.Microphone, PermissionName.Camera];
      final permissions = await Permission.getPermissionsStatus(permissionList);
      _cameraPermissionStatus = permissions[1].permissionStatus;
      _microphonePermissionStatus = permissions[0].permissionStatus;
    } else if (Platform.isIOS) {
      _cameraPermissionStatus =
          await Permission.getSinglePermissionStatus(PermissionName.Camera);
      _microphonePermissionStatus =
          await Permission.getSinglePermissionStatus(PermissionName.Microphone);
    } else {
      // Handle error
    }

    setState(() {
      _loading = false;
    });
  }

  @override
  Widget build(BuildContext context) {
//    if (_loading) {
//      return MaterialApp(
//        home: Scaffold(
//          body: Center(child: CircularProgressIndicator()),
//        ),
//      );
//    }
//
//    if (!_hasRequiredPermissions) {
//      Widget action;
//      if (_cameraPermissionStatus == null ||
//          _microphonePermissionStatus == null) {
//        action = Text(
//          'App is not supported for this device',
//          textAlign: TextAlign.center,
//        );
//      } else if (_cameraPermissionStatus == PermissionStatus.notDecided ||
//          _microphonePermissionStatus == PermissionStatus.notDecided) {
//        action = RaisedButton(
//          child: Text("Review permissions"),
//          onPressed: () async {
//            if (_cameraPermissionStatus == PermissionStatus.notDecided) {
//              _cameraPermissionStatus = await Permission.requestSinglePermission(PermissionName.Camera);
//            }
//            if (_microphonePermissionStatus == PermissionStatus.notDecided) {
//              _microphonePermissionStatus = await Permission.requestSinglePermission(PermissionName.Microphone);
//            }
//
//            setState(() {
//
//            });
//          },
//        );
//      } else {
//        action = RaisedButton(
//          child: Text("Open settings"),
//          onPressed: () async {
//            await Permission.openSettings();
//          },
//        );
//      }
//      return MaterialApp(
//        home: Scaffold(
//          appBar: AppBar(
//            title: Text('JustGo'),
//          ),
//          body: Container(
//            margin: EdgeInsets.symmetric(horizontal: 20, vertical: 35),
//            child: Center(
//              child: Column(
//                children: <Widget>[
//                  Text(
//                    'App does not have all required permissions to function properly.',
//                    textAlign: TextAlign.center,
//                  ),
//                  Container(
//                    margin: EdgeInsets.symmetric(vertical: 10),
//                    child: Text(
//                      'Please allow camera and microphone usage by enabling them.',
//                      textAlign: TextAlign.center,
//                    ),
//                  ),
//                  action,
//                ],
//              ),
//            ),
//          ),
//        ),
//      );
//    }

    return MaterialApp(
      title: 'JustGo',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      initialRoute: '/',
      onGenerateRoute: (settings) {
        switch (settings.name) {
          case '/':
            return MaterialPageRoute(builder: (_) => LandingPage());
            break;
          case '/home':
            return MaterialPageRoute(builder: (_) => HomePage());
            break;
          case '/register':
            final uuid = settings.arguments;
            return MaterialPageRoute(builder: (_) => RegisterPage(uuid: uuid));
            break;
          default:
            return null;
        }
      },
    );
  }
}
