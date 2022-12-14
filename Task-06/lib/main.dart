import 'package:flame/components.dart';
import 'package:flame/game.dart';
// ignore: unused_import
import 'package:flame/image_composition.dart';
// ignore: unused_import
import 'package:flame/input.dart';
import 'package:flutter/material.dart' hide Image;
import 'bunnygame.dart';
// ignore: unused_import
import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'helpers/navigation_keys.dart';

void main() {
  final game = BunnyGame();
  runApp(
    MaterialApp(
      debugShowCheckedModeBanner: false,
      home: Scaffold(
        body: Stack(
          children: [
            GameWidget(
              game: game,
            ),
            Align(
              alignment: Alignment.bottomRight,
              child: NavigationKeys(
                onDirectionChanged: game.onArrowKeyChanged,
              ),
            ),
          ],
        ),
      ),
    ),
  );
}
