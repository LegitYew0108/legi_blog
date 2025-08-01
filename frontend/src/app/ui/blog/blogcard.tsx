import CardData from '@/app/lib/definitions'
import Image from 'next/image'
import {parseJSON, format} from 'date-fns'
import { ja } from 'date-fns/locale';


export default function BlogCard({article}: {article:CardData}){
	return<div className="relative rounded-4xl m-[2rem] border-2 border-black bg-white transition transition-transform duration-150 hover:scale-105 hover:z-10">
		<div className="relative rounded-t-4xl h-[60%] overflow-hidden">
			<Image src={article.image_url} alt={article.title} fill={true}
			className="relative! object-cover!" />
		</div>
		<p className="text-gray-400">{format(parseJSON(article.date),"yyyy/MM/dd HH:mm ",{locale: ja})}</p>
		<h1 className="text-[3rem]">
			{article.title}
		</h1>
		<div className="ml-[2rem] mr-[2rem]">
			{article.abstract}
		</div>
	</div>;
}

export function BlogCards({articles}: {articles:CardData[]}){
	if (!articles || articles.length === 0) {
		return <p>記事が見つかりませんでした。</p>;
	}
	return <>{
		articles.map((article)=>{
			return <BlogCard key={article.article_id} article={article} />
		})
	}
	</>
}
