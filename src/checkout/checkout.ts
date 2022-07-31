import { Discount } from "./Discount";
import { itemFactory } from "./Item";

export class Store {
    constructor(private discountList?: Discount[]) { }

    checkout(itemList: string): number {
        const total = this.caclulateTotal(itemList);
        const discount = this.calculateDiscount(itemList);

        return total - discount;
    }

    private calculateDiscount(itemList: string) {
        let totalDiscount = 0;
        if (this.discountList === undefined) {
            return totalDiscount;
        }

        for (const discount of this.discountList) {
            totalDiscount += discount.calculateDiscount(itemList);
        }
        return totalDiscount;
    }

    private caclulateTotal(itemList: string) {
        let total = 0;
        for (const itemChar of itemList.split("")) {
            const item = itemFactory(itemChar);
            total += item.getPrice();
        }
        return total;
    }
}

