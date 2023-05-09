// Uncomment the following lines when enabling Firebase Crashlytics
// import 'dart:io';
// import 'package:firebase_core/firebase_core.dart';
// import 'firebase_options.dart';

import 'package:firebase_crashlytics/firebase_crashlytics.dart';
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:provider/provider.dart';

import 'src/app_lifecycle/app_lifecycle.dart';
import 'src/crashlytics/crashlytics.dart';
import 'src/style/palette.dart';

Future<void> main() async {
  // To enable Firebase Crashlytics, uncomment the following lines and
  // the import statements at the top of this file.
  // See the 'Crashlytics' section of the main README.md file for details.

  FirebaseCrashlytics? crashlytics;
  // if (!kIsWeb && (Platform.isIOS || Platform.isAndroid)) {
  //   try {
  //     WidgetsFlutterBinding.ensureInitialized();
  //     await Firebase.initializeApp(
  //       options: DefaultFirebaseOptions.currentPlatform,
  //     );
  //     crashlytics = FirebaseCrashlytics.instance;
  //   } catch (e) {
  //     debugPrint("Firebase couldn't be initialized: $e");
  //   }
  // }

  await guardWithCrashlytics(
    guardedMain,
    crashlytics: crashlytics,
  );
}

void guardedMain() {
  runApp(MyApp());
}

//Logger _log = Logger('main.dart');

class MyApp extends StatelessWidget {
  static final _router = GoRouter(
    routes: [
      GoRoute(
          path: '/',
          builder: (context, state) => MyHomePage(),
          routes: const []
      ),
    ]
  );

  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return AppLifecycleObserver(
      child: MultiProvider(
        providers: [
          ChangeNotifierProvider(
            create: (context) => MyAppState(),
          ),
          Provider(
            create: (context) => Palette(),
          )
        ],
        child: Builder(builder: (context) {
          final palette = context.watch<Palette>();

          // TODO why isn't my background yellow... ?
          return MaterialApp.router(
            title: 'BirdTalk',
            theme: ThemeData.from(
              useMaterial3: true,
              colorScheme: ColorScheme.fromSeed(
                  seedColor: palette.darkPen,
                  background: palette.backgroundMain,
              ),
              textTheme: TextTheme(
                bodyMedium: TextStyle(
                  color: palette.ink,
                  fontFamily: 'Permanent Marker'
                ),
              ),
            ),
            routeInformationProvider: _router.routeInformationProvider,
            routeInformationParser: _router.routeInformationParser,
            routerDelegate: _router.routerDelegate,
          );
        }),
      )
    );
  }
}

class MyAppState extends ChangeNotifier {
  var current = "BirdTalk";
  void getNext() {
    current = "Talk Birdy To Me";
    notifyListeners();
  }

  Set<String> favorites = {};

  void toggleFavorite() {
    if (favorites.contains(current)) {
      favorites.remove(current);
    } else {
      favorites.add(current);
    }
    notifyListeners();
  }
}

class MyHomePage extends StatefulWidget {
  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  var selectedIndex = 0;

  @override
  Widget build(BuildContext context) {

    Widget page;
    switch (selectedIndex) {
      case 0:
        page = GeneratorPage();
        break;
      case 1:
        page = FavoritesPage();
        break;
      default:
        throw UnimplementedError('no widget for $selectedIndex');
    }

    return LayoutBuilder(
      builder: (context, constraints) {
        return Scaffold(
            body: Row(
                children: [
                  SafeArea(
                      child: NavigationRail(
                        extended: constraints.maxWidth >= 600,
                        destinations: [
                          NavigationRailDestination(
                            icon: Icon(Icons.home),
                            label: Text('Home'),
                          ),
                          NavigationRailDestination(
                            icon: Icon(Icons.favorite),
                            label: Text('Favorites'),
                          )
                        ],
                        selectedIndex: selectedIndex,
                        onDestinationSelected: (value) {
                          setState(() {
                            selectedIndex = value;
                          });
                        },
                    ),
                  ),
                  Expanded(
                      child: Container(
                          color: Theme.of(context).colorScheme.primaryContainer,
                          child: page,
                      ),
                  ),
                ],
              ),
            );
      }
    );
  }
}

class GeneratorPage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    var appState = context.watch<MyAppState>();
    var pair = appState.current;
    var favorited = appState.favorites.contains(pair);

    return Center(
      child: Column(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        BigCard(pair: pair),
        SizedBox(height: 15),
        Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            ElevatedButton.icon(
              onPressed: () {
                appState.toggleFavorite();
              },
              icon: Icon(favorited ? Icons.favorite : Icons.favorite_outline),
              label: Text('Like'),
            ),
            SizedBox(width: 15),
            ElevatedButton(
              onPressed: () {
                appState.getNext();
              },
              child: Text('Next'),
            ),
          ],
        ),
      ],
    ));
  }
}

class BigCard extends StatelessWidget {
  const BigCard({
    super.key,
    required this.pair,
  });

  final String pair;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    final style = theme.textTheme.displayMedium!.copyWith(
      color: theme.colorScheme.onPrimary,
      letterSpacing: 1.0,
      // TODO figure out how to propagate this font through theme
      fontFamily: 'Permanent Marker',
      fontSize: 50
    );
    return Card(
      color: theme.colorScheme.tertiary,
      elevation: 5.0,
      child: Padding(
        padding: const EdgeInsets.all(20.0),
        child: Text(
          pair,
          style: style,
          semanticsLabel: "${pair} ${pair}",
        ),
      ),
    );
  }
}

class FavoritesPage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    var appState = context.watch<MyAppState>();
    var favorites = appState.favorites;

    if (favorites.isEmpty) {
      return Center(
          child: Text('None yet!')
      );
    }

    return Center(
      child: ListView(
        children: [
          Padding(
            padding: const EdgeInsets.all(20),
            child: Text('You have ${favorites.length} favorites:'),
          ),
          for (var pair in favorites)
            ListTile(
                leading: Icon(Icons.favorite),
                title: Text(pair),
            ),
        ]
      ),
    );
  }
}
