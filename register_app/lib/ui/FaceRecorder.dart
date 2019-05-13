import 'dart:async';
import 'dart:io';

import 'package:camera/camera.dart';
import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:path_provider/path_provider.dart';

class FaceRecorder extends StatefulWidget {
  List<CameraDescription> cameras;

  FaceRecorder({@required this.cameras});

  @override
  State<StatefulWidget> createState() => _FaceRecorderState(cameras: cameras);
}

class _FaceRecorderState extends State<FaceRecorder> {
  CameraController _controller;
  String _videoPath;
  List<CameraDescription> cameras;
  CameraDescription camera;
  bool loading = true;
  bool capturing = false;
  int captureTimeSeconds = 0;
  final int maxCaptureTime = 5;
  Timer recordingTimer;

  _FaceRecorderState({@required this.cameras});

  @override
  void initState() {
    super.initState();

    initialize();
  }

  void initialize() async {
    if (cameras.length == 0) {
      // Can't show anything without a camera
      this.setState(() {
        loading = false;
      });
      return;
    }

    // Choose a front facing camera as priority
    final frontFacingCameras = cameras
        .where((x) => x.lensDirection == CameraLensDirection.front)
        .toList();
    if (frontFacingCameras.length > 0) {
      this.camera = frontFacingCameras[0];
    } else {
      this.camera = cameras[0];
    }

    _controller = CameraController(camera, ResolutionPreset.medium);
    await _controller.initialize();

    this.setState(() {
      loading = false;
    });
  }

  @override
  void dispose() {
    _controller?.dispose();
    super.dispose();
  }

  void _handleCaptureButtonTapUp(TapUpDetails details) async {
    if (capturing) {
      if (!_controller.value.isRecordingVideo) {
        return;
      }

      try {
        await _controller.stopVideoRecording();
      } catch (e) {
        // handle error
        return;
      }
    } else {
      final dir = await getApplicationDocumentsDirectory();
      final dirPath = '${dir.path}/Movies/face_recording';
      await Directory(dirPath).create(recursive: true);
      final filePath = '$dirPath/recording.mp4';
      final file = File(filePath);
      if (await file.exists()) {
        await file.delete();
      }

      try {
        await _controller.startVideoRecording(filePath);
        if (recordingTimer != null) {
          recordingTimer.cancel();
        }
        recordingTimer = Timer.periodic(Duration(milliseconds: 1000), handleTimerTick);
      } catch (e) {
        // handle error
        return;
      }
    }

    setState(() {
      capturing = !capturing;
    });
  }

  void handleTimerTick(Timer timer) {
    setState(() {
      captureTimeSeconds++;
      if (captureTimeSeconds > maxCaptureTime) {
        // Finished
        timer.cancel();

        // Save
      }
      capturing = false;
    });
  }

  Widget renderCaptureButton(BuildContext context) {
    return ClipOval(
      child: Container(
        color: capturing ? Colors.red : Colors.white,
        height: 75,
        width: 75,
        padding: EdgeInsets.all(5),
        child: ClipOval(
          child: Container(
            color: Colors.black,
            padding: EdgeInsets.all(3),
            child: GestureDetector(
              onTapUp: _handleCaptureButtonTapUp,
              child: ClipOval(
                child: Container(
                  color: Colors.red,
                  child: capturing
                      ? Center(
                          child: Text(
                            "${maxCaptureTime - captureTimeSeconds}",
                            style: TextStyle(color: Colors.white, fontSize: 31),
                          ),
                        )
                      : null,
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    if (loading) {
      return Center(child: CircularProgressIndicator());
    }

    if (camera == null) {
      return Center(
        child: Text(
          "No camera available",
          textAlign: TextAlign.center,
        ),
      );
    }

    final stackChildren = <Widget>[
      Positioned(
        child: CameraPreview(_controller),
      ),
      Positioned(
        child: Container(
          alignment: Alignment.bottomCenter,
          margin: EdgeInsets.only(bottom: 75),
          child: renderCaptureButton(context),
        ),
      ),
    ];

    if (capturing) {
      stackChildren.add(Positioned(
        child: Container(
          color: Color.fromARGB(100, 255, 255, 255),
          margin: EdgeInsets.only(top: 20, left: 20),
          padding: EdgeInsets.symmetric(horizontal: 5, vertical: 5),
          child: Text("Recording..."),
        ),
      ));
    }

    return Container(
      child: Stack(children: stackChildren),
    );
  }
}
