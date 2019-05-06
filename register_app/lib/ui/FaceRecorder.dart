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
  bool capturing = false;

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

  void _handleCaptureButtonTapUp(TapUpDetails details) {
    setState(() {
      capturing = !capturing;
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
                child: Container(color: Colors.red),
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

    return Container(
      child: Stack(
        children: <Widget>[
          Positioned(
            child: CameraPreview(_controller),
          ),
          Positioned(
            child: Container(
                alignment: Alignment.bottomCenter,
                margin: EdgeInsets.only(bottom: 75),
                child: renderCaptureButton(context)),
          ),
        ],
      ),
    );
  }
}
