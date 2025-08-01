
export default function BlogHeader(){
	return<header className="flex flex-row items-center bg-gray-400 h-[4rem]">
		<div className="text-3xl">れぎっとの技術日記(ブログ)</div>
		<ArticleNum />
		<SearchForm />
	</header>
}

export function SearchForm(){
	return<form className="relative rounded-full ml-[3rem] bg-white w-[18rem] h-[2rem]">
			<div className="absolute left-[0.3rem] top-1">
				<svg xmlns="http://www.w3.org/2000/svg" className="w-[1.5rem] h-[1.5rem]" viewBox="0 0 29 29" fill="none">
				<path d="M26.875 26.875L20.8937 20.8937M24.125 13.125C24.125 19.2001 19.2001 24.125 13.125 24.125C7.04987 24.125 2.125 19.2001 2.125 13.125C2.125 7.04987 7.04987 2.125 13.125 2.125C19.2001 2.125 24.125 7.04987 24.125 13.125Z" stroke="#1E1E1E" strokeWidth="4" strokeLinecap="round" strokeLinejoin="round"/>
				</svg>
			</div>
			<input id="search-input" type="search" placeholder="記事を検索..." className="absolute top-1 left-[2rem] w-[]"/>
		</form>
}

export function ArticleNum(){
	const articleNum = 13;
	return<div className="ml-auto">
		全記事数：{articleNum}
	</div>
}
