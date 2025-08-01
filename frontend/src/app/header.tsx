'use client';

import Link from 'next/link'
import Image from 'next/image'
import clsx from 'clsx'
import {usePathname} from 'next/navigation';

export default function GlobalHeader(){
	return<header className="flex flex-row items-end bg-white">
		<Image src="/legit.png" alt="legit logo" width={200} height={50} />
		<div className="relative">
			<p className="absolute z-0 bottom-[2rem] left-[8rem] text-[2rem] text-gray-400">Legit</p>
			<p className="relative text-black z-10 text-[3rem]">れぎっと</p>
		</div>
		<GlobalHeaderNavs />
	</header>
}

const links = [
  { name: 'Home', href: '/'},
  { name: 'Career',href: '/career'},
  { name: 'Blog', href: '/blog' },
	{ name: 'Contact', href: '/contact'},
];

export function GlobalHeaderNavs(){
	const pathname = usePathname();
	return <nav className="ml-auto">
			<ul className="flex flex-row">
				{links.map((link) => {
						return(<Link key={link.name} href={link.href} className="relative group mr-[1.5rem] ml-[1.5rem] text-[2rem]" >
									 {link.name}
							<div className={clsx("absolute bg-gray-400 bottom-[0.3rem] left-[calc(50%-4rem/2)] w-[4rem] h-[0.3rem] group-hover:bg-sky-500",{
								"bg-sky-500": pathname==link.href
							})}>
							</div>
						</Link>
						);
					})
				}
			</ul>
		</nav>
}
