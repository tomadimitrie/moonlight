//
//  AppDelegate.swift
//  moonlight
//
//  Created by Dimitrie-Toma Furdui on 06/10/2020.
//

import UIKit

var globalTree: View?

@_cdecl("render")
func render(_ tree: UnsafePointer<NativeView>) {
    globalTree = View.from(pointer: tree)
}

@main
class AppDelegate: UIResponder, UIApplicationDelegate {
    func application(_ application: UIApplication, didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?) -> Bool {
        run_app()
        print(globalTree)
        end_app(globalTree?.pointer)
        return true
    }
    
    func application(_ application: UIApplication, configurationForConnecting connectingSceneSession: UISceneSession, options: UIScene.ConnectionOptions) -> UISceneConfiguration {
        return UISceneConfiguration(name: "Default Configuration", sessionRole: connectingSceneSession.role)
    }
}

