import 'package:flutter/material.dart';
import 'package:register_app/pages/HomePage.dart';
import 'package:register_app/pages/LandingPage.dart';

void main() => runApp(MyApp());

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'JustGo',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      initialRoute: '/',
      routes: {
        '/': (_) => LandingPage(),
        '/home': (_) => HomePage()
      }
    );
  }
}
