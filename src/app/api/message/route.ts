import { create_message } from "@/message-board/redis";
import { NextResponse } from "next/server";

export async function POST(request: Request) {
	let { username = "", message = "" } = await request.json();
	username = String(username).trim();
	message = String(message).trim();
	
	if (!username || !message) {
		return NextResponse.json({
			message: "No message or username specified"
		}, {
			status: 400
		});
	}

	create_message(username.substring(0, 10), message.substring(0, 150));
	return NextResponse.json({ message: "OK" });
}