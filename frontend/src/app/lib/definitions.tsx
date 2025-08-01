
export default class CardData{
	public article_id: string;
	public image_url: string;
	public title: string;
	public date: string;
	public abstract: string;

	constructor(article_id: string, image_url: string, title: string, date: string, abstract: string){
		this.article_id = article_id;
		this.image_url = image_url;
		this.title = title;
		this.date = date;
		this.abstract = abstract;
	}
}
