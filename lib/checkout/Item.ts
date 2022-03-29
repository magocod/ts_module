import { A } from "./A";
import { B } from "./B";
import { C } from "./C";
import { D } from "./D";

export interface Item {
    getPrice(): number;
}

export function itemFactory(itemChar: string): Item {
    if (itemChar === ItemType.D) return new D();
    if (itemChar === ItemType.C) return new C();
    if (itemChar === ItemType.B) return new B();
    if (itemChar === ItemType.A) return new A();
    throw new Error("Unknown Item");
}

export enum ItemType {
    A = "A",
    B = "B",
    C = "C",
    D = "D"
}