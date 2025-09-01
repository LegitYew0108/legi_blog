import {BlogCards} from '@/app/ui/blog/blogcard'
import BlogHeader from '../ui/blog/blog-header';
import { CardSortMethod } from '../lib/definitions';


export default function BlogHome(){
	const back_url = "http://localhost:3440";
	return(
		<div>
			<BlogHeader />
			<main>
				<div className="mr-[calc((100%-70rem)/2)] ml-[calc((100%-70rem)/2)]">
					<div className="m-[1.5rem] text-2xl">
					最近書いた記事
					</div>
					<div className="grid grid-cols-[repeat(2,1fr)]">
						<BlogCards back_base_url={back_url} sort_method={CardSortMethod.Latest} tag_id={null} card_num={4} />
					</div>
					<div className="m-[1.5rem] text-2xl">
					ロボティクス/ハードウェア
					</div>
					<div className="grid grid-cols-[repeat(2,1fr)]">
						<BlogCards back_base_url={back_url} sort_method={CardSortMethod.Latest} tag_id={null} card_num={4} />
					</div>
					<div className="m-[1.5rem] text-2xl">
					OS/低レイヤ
					</div>
					<div className="grid grid-cols-[repeat(2,1fr)]">
						<BlogCards back_base_url={back_url} sort_method={CardSortMethod.Latest} tag_id={null} card_num={4} />
					</div>
					<div className="m-[1.5rem] text-2xl">
					Web/ネットワーク
					</div>
					<div className="grid grid-cols-[repeat(2,1fr)]">
						<BlogCards back_base_url={back_url} sort_method={CardSortMethod.Latest} tag_id={null} card_num={4} />
					</div>
					<div className="m-[1.5rem] text-2xl">
					雑多/ネタ記事
					</div>
					<div className="grid grid-cols-[repeat(2,1fr)]">
						<BlogCards back_base_url={back_url} sort_method={CardSortMethod.Latest} tag_id={null} card_num={4} />
					</div>
				</div>
			</main>
		</div>
	);
}

