import CardData from '@/app/lib/definitions';
import axios from "axios";

export default async function fetchCards(back_base_url:string){
	console.log("fetch data start:");
	const url = back_base_url + "/cards";
	const query = "?method=Latest&card_num=3";
	const response = await axios.get(url+query);
	const cards: Array<CardData> = JSON.parse(JSON.stringify(response.data));
	return cards;
}
