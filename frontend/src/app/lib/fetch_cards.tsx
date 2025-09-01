import { CardSortMethod } from "./definitions";
import CardData from '@/app/lib/definitions';
import axios from "axios";

export default async function fetchCards(back_base_url:string, sort_method:CardSortMethod, tag_id:string|null, card_num:number){
	console.log("fetch data start:");
	const url = back_base_url + "/cards";
	let method = "Latest";
	if (sort_method==CardSortMethod.Tag){
		method = "Latest&tag_id="+tag_id;
	}
	const query = "?method="+method+"&card_num="+card_num;
	const response = await axios.get(url+query);
	const cards: Array<CardData> = JSON.parse(JSON.stringify(response.data));
	return cards;
}
