//
//  Moonlight.swift
//  moonlight
//
//  Created by Dimitrie-Toma Furdui on 06/10/2020.
//

import Foundation

struct View {
    let attributes: [String]
    let children: [Self]
    let pointer: UnsafePointer<NativeView>
}

extension View {
    static func from(pointer: UnsafePointer<NativeView>) -> Self {
        let attributes = Array(UnsafeBufferPointer(
            start: pointer.pointee.attributes_ptr,
            count: Int(pointer.pointee.attributes_size)
        )).map { attribute -> String in
            let cString = unsafeBitCast(attribute, to: UnsafePointer<CChar>.self)
            let string = String(cString: cString)
            return string
        }
        let children = Array(UnsafeBufferPointer(
            start: pointer.pointee.children_ptr,
            count: Int(pointer.pointee.children_size)
        )).map { child in
            Self.from(pointer: child!)
        }
        return View(attributes: attributes, children: children, pointer: pointer)
    }
}
