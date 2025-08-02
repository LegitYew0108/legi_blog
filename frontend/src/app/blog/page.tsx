import {BlogCards} from '@/app/ui/blog/blogcard'
import BlogHeader from '../ui/blog/blog-header';


export default function BlogHome(){
	const sampleArticles = [
		{article_id:"0001", image_url:"https://placehold.jp/1920x1080.png",title:"タイトル",timestamp:"2000-03-15T05:20:10.123Z",abstract:"おはよう。"},
	]
	return(
		<div>
			<BlogHeader />
			<main>
				<div className="mr-[calc((100%-70rem)/2)] ml-[calc((100%-70rem)/2)]">
					<div className="m-[1.5rem] text-2xl">
					最近書いた記事
					</div>
					<div className="grid grid-cols-[repeat(2,1fr)]">
						<BlogCards />
					</div>
					<div className="m-[1.5rem] text-2xl">
					ロボティクス/ハードウェア
					</div>
					<div className="grid grid-cols-[repeat(2,1fr)]">
						<BlogCards />
					</div>
					<div className="m-[1.5rem] text-2xl">
					OS/低レイヤ
					</div>
					<div className="grid grid-cols-[repeat(2,1fr)]">
						<BlogCards />
					</div>
					<div className="m-[1.5rem] text-2xl">
					Web/ネットワーク
					</div>
					<div className="grid grid-cols-[repeat(2,1fr)]">
						<BlogCards />
					</div>
					<div className="m-[1.5rem] text-2xl">
					雑多/ネタ記事
					</div>
					<div className="grid grid-cols-[repeat(2,1fr)]">
						<BlogCards />
					</div>
				</div>
			</main>
		</div>
	);
}

