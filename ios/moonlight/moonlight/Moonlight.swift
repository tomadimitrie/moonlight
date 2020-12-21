//
//  Moonlight.swift
//  moonlight
//
//  Created by Dimitrie-Toma Furdui on 06/10/2020.
//

import Foundation

struct View {
    let attributes: [Attribute<AttributeRepresentable>]
    let children: [Self]
    let pointer: UnsafePointer<NativeView>
    let isWrapper: Bool
}

protocol AttributeRepresentable {}

extension String: AttributeRepresentable {}
extension Int: AttributeRepresentable {}

struct Attribute<T> {
    let name: String
    let value: T
}

extension View {
    static func from(pointer: UnsafePointer<NativeView>) -> Self {
        let attributesArray = Array(UnsafeBufferPointer(
            start: pointer.pointee.attributes_ptr,
            count: Int(pointer.pointee.attributes_size)
        ))
        let attributes = stride(from: 0, to: pointer.pointee.attributes_size, by: 3).map { index -> Attribute<AttributeRepresentable> in
            let index = Int(index)
            let namePtr = attributesArray[index]
            let valuePtr = attributesArray[index.advanced(by: 1)]
            let typePtr = attributesArray[index.advanced(by: 2)]
            
            let type = String(cString: unsafeBitCast(typePtr, to: UnsafePointer<CChar>.self))
            let name = String(cString: unsafeBitCast(namePtr, to: UnsafePointer<CChar>.self))
            let value: AttributeRepresentable
            switch type {
                case "String":
                    value = String(cString: unsafeBitCast(valuePtr, to: UnsafePointer<CChar>.self))
                case "usize":
                    value = unsafeBitCast(valuePtr, to: UnsafePointer<Int>.self).pointee
                default:
                    fatalError("the type \(type) not supported yet")
            }
            
            return Attribute(name: name, value: value)
        }
        let children = Array(UnsafeBufferPointer(
            start: pointer.pointee.children_ptr,
            count: Int(pointer.pointee.children_size)
        )).map { child in
            Self.from(pointer: child!)
        }
        return View(attributes: attributes, children: children, pointer: pointer, isWrapper: pointer.pointee.is_wrapper)
    }
}
