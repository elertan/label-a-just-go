import 'package:camera/camera.dart';
import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';

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
    final frontFacingCameras = cameras.where((x) => x.lensDirection == CameraLensDirection.front).toList();
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

  @override
  Widget build(BuildContext context) {
    if (loading) {
      return Center(
        child: CircularProgressIndicator()
      );
    }

    if (camera == null) {
      return Center(
        child: Text(
          "No camera available",
          textAlign: TextAlign.center,
        ),
      );
    }

    return Container(
      child: CameraPreview(_controller),
    );
  }
}