
export default interface ArticleMetadata{
	title: string;
	timestamp: string;
	abstract_sentense: string;
	main_image: string;
}

export default interface CardData{
	id: string;
	metadata: ArticleMetadata;
}
