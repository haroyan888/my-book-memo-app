import {Button} from "react-bootstrap";
import {useState} from "react";

interface props {
	text: string,
}

export default function ShowMore({text}: props) {
	const [isShowMore, setIsShowMore] = useState<boolean>(false);

	const maxCharLen = 170;
	return (
		<div style={{height: "fit-content", marginBottom: "10px"}}>
			<p style={{overflow: "hidden", maxHeight: !isShowMore?"100px":"fit-content", height: "fit-content"}}>
				{text.length <= maxCharLen || isShowMore
					? text
					: text.slice(0, maxCharLen) + '...'}
			</p>
			{text.length > maxCharLen
				? <Button onClick={() => setIsShowMore(!isShowMore)}>
						{!isShowMore ? "もっと見る" : "折りたたむ"}
					</Button>
				: undefined}
		</div>
	);
}