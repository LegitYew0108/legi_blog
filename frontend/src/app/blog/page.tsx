import {BlogCards} from '@/app/ui/blog/blogcard'
import BlogHeader from '../ui/blog/blog-header';
import CardData from '../lib/definitions';


export default function BlogHome(){
	const sampleArticles = [
		new CardData("0001", "https://placehold.jp/1920x1080.png","タイトル","2000-03-15T05:20:10.123Z","おはよう。"),
		new CardData("0002", "https://placehold.jp/1920x1080.png","タイトル","2000-03-15T05:20:10.123Z","こんにちは。"),
		new CardData("0003", "https://placehold.jp/1920x1080.png","タイトル","2000-03-15T05:20:10.123Z","こんばんは。"),
		new CardData("0004", "https://placehold.jp/1920x1080.png","タイトル","2000-03-15T05:20:10.123Z","おやすみ。"),
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
						<BlogCards articles={sampleArticles} />
					</div>
					<div className="m-[1.5rem] text-2xl">
					ロボティクス/ハードウェア
					</div>
					<div className="grid grid-cols-[repeat(2,1fr)]">
						<BlogCards articles={sampleArticles} />
					</div>
					<div className="m-[1.5rem] text-2xl">
					OS/低レイヤ
					</div>
					<div className="grid grid-cols-[repeat(2,1fr)]">
						<BlogCards articles={sampleArticles} />
					</div>
					<div className="m-[1.5rem] text-2xl">
					Web/ネットワーク
					</div>
					<div className="grid grid-cols-[repeat(2,1fr)]">
						<BlogCards articles={sampleArticles} />
					</div>
					<div className="m-[1.5rem] text-2xl">
					雑多/ネタ記事
					</div>
					<div className="grid grid-cols-[repeat(2,1fr)]">
						<BlogCards articles={sampleArticles} />
					</div>
				</div>
			</main>
		</div>
	);
}

