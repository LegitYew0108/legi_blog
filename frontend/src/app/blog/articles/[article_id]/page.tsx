import axios from "axios";
import parse from 'html-react-parser';

const base_url = 'http://localhost:3440/';

export default async function Page({params}: {params: {article_id: string}}){
	const query = 'articles/' + params.article_id;
	const response = await axios.get(base_url+query);

	return <div>{parse(response.data)}</div>;
}
