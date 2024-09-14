export default interface Book {
	isbn_13: string,
	title: string,
	description: string,
	authors: string[],
	publisher: string,
	published_date: string,
	image_url: string,
};