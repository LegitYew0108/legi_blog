import CardData, { CardSortMethod } from '@/app/lib/definitions'
import Image from 'next/image'
import {parseJSON, format} from 'date-fns'
import { ja } from 'date-fns/locale';
import fetchCards from '@/app/lib/fetch_cards'


export default async function BlogCard({article}: {article:CardData}){
	let image_data = article.metadata.main_image;
	if (image_data == null){
		image_data = "/article_placeholder.png";
	}
	return<a href={"blog/articles/"+article.id} className="relative rounded-4xl m-[2rem] border-2 border-black bg-white transition transition-transform duration-150 hover:scale-105 hover:z-10">
		<div className="relative rounded-t-4xl h-[60%] overflow-hidden">
			<Image src={image_data} alt={article.metadata.title} fill={true}
			className="relative! object-cover!" />
		</div>
		<p className="text-gray-400">{format(parseJSON(article.metadata.timestamp),"yyyy/MM/dd HH:mm ",{locale: ja})}</p>
		<h1 className="text-[3rem]">
			{article.metadata.title}
		</h1>
		<div className="ml-[2rem] mr-[2rem]">
			{article.metadata.abstract_sentense}
		</div>
	</a>;
}

export async function BlogCards(
	{back_base_url,sort_method,tag_id,card_num}:{back_base_url:string, sort_method:CardSortMethod, tag_id:string|null, card_num:number}){
	const articles:Array<CardData> = await fetchCards(back_base_url,sort_method,tag_id,card_num);
	if (!articles || articles.length === 0) {
		return <p>記事が見つかりませんでした。</p>;
	}
	return <>{
		articles.map((article)=>{
			return <BlogCard key={article.id} article={article} />
		})
	}
	</>
}
