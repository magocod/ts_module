import { Item } from "./Item";

const A_PRICE = 50;

export class A implements Item {
    getPrice(): number {
        return A_PRICE;
    }
}