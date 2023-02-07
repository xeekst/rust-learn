use encoding_rs::*;

//https://learn.microsoft.com/en-us/windows/win32/intl/code-page-identifiers
//https://github.com/lifthrasiir/rust-encoding/blob/eb3d3c307df864f6a25e2ca16d49703e5d963ec5/src/label.rs

pub fn encoding_from_codepage(codepage: u16) -> Option<&'static Encoding> {
    match codepage {
		037 | //IBM EBCDIC US-Canada
		437     => Option::Some(&UTF_8_INIT),//OEM United States
		708 | //Arabic (ASMO 708)
		709 | //Arabic (ASMO-449+, BCON V4)
		710 | //Arabic - Transparent Arabic
		720     => Option::Some(&WINDOWS_1256_INIT), //Arabic (Transparent ASMO); Arabic (DOS)
		737     => Option::Some(&ISO_8859_7_INIT), //OEM Greek (formerly 437G); Greek (DOS)
		775     => Option::Some(&WINDOWS_1257_INIT), //OEM Baltic; Baltic (DOS)
		850     => Option::Some(&WINDOWS_1252_INIT), //OEM Multilingual Latin 1; Western European (DOS)
		852	    => Option::Some(&ISO_8859_2_INIT), //OEM Latin 2; Central European (DOS)
		855	    => Option::Some(&ISO_8859_5_INIT), //OEM Cyrillic (primarily Russian)
		857 	=> Option::Some(&WINDOWS_1254_INIT), //OEM Turkish; Turkish (DOS)
		858	    => Option::Some(&WINDOWS_1252_INIT), //OEM Multilingual Latin 1 + Euro symbol
		860 	=> Option::Some(&ISO_8859_15_INIT), //OEM Portuguese; Portuguese (DOS)
		861 	=> Option::Some(&WINDOWS_1257_INIT), //OEM Icelandic; Icelandic (DOS)
		862	    => Option::Some(&ISO_8859_8_INIT), //OEM Hebrew; Hebrew (DOS)
		863 	=> Option::Some(&WINDOWS_1252_INIT), //OEM French Canadian; French Canadian (DOS)
		864     => Option::Some(&WINDOWS_1256_INIT), //OEM Arabic; Arabic (864)
		865	    => Option::Some(&ISO_8859_4_INIT), //	OEM Nordic; Nordic (DOS)
		866	    => Option::Some(&IBM866_INIT), //OEM Russian; Cyrillic (DOS)
		869	    => Option::Some(&ISO_8859_7_INIT), //OEM Modern Greek; Greek, Modern (DOS)
		870 	=> Option::Some(&ISO_8859_2_INIT), //IBM EBCDIC Multilingual/ROECE (Latin 2); IBM EBCDIC Multilingual Latin 2
		874	    => Option::Some(&WINDOWS_874_INIT), //Thai (Windows)
		875	    => Option::Some(&ISO_8859_2_INIT), //	IBM EBCDIC Greek Modern
		932	    => Option::Some(&SHIFT_JIS_INIT), //ANSI/OEM Japanese; Japanese (Shift-JIS)
		936		=> Option::Some(&GBK_INIT), //ANSI/OEM Simplified Chinese (PRC, Singapore); Chinese Simplified (GB2312)
		949		=> Option::Some(&EUC_KR_INIT), //ANSI/OEM Korean (Unified Hangul Code)
		950 	=> Option::Some(&BIG5_INIT), //ANSI/OEM Traditional Chinese (Taiwan; Hong Kong SAR, PRC); Chinese Traditional (Big5)
		951		=> Option::Some(&BIG5_INIT),
		1026    => Option::Some(&WINDOWS_1254_INIT) , //	IBM EBCDIC Turkish (Latin 5)
		1047	=> Option::Some(&WINDOWS_1252_INIT), // IBM EBCDIC Latin 1/Open System
		1140	=> Option::Some(&UTF_8_INIT)	, // 	IBM EBCDIC US-Canada (037 + Euro symbol); IBM EBCDIC (US-Canada-Euro)
		1141	=> Option::Some(&WINDOWS_1252_INIT), //IBM EBCDIC Germany (20273 + Euro symbol); IBM EBCDIC (Germany-Euro)
		1142	=> Option::Some(&ISO_8859_4_INIT), //IBM EBCDIC Denmark-Norway (20277 + Euro symbol); IBM EBCDIC (Denmark-Norway-Euro)
		1143	=> Option::Some(&WINDOWS_1252_INIT), //IBM EBCDIC Finland-Sweden (20278 + Euro symbol); IBM EBCDIC (Finland-Sweden-Euro)
		1144	=> Option::Some(&ISO_8859_3_INIT), //IBM EBCDIC Italy (20280 + Euro symbol); IBM EBCDIC (Italy-Euro)
		1145	=> Option::Some(&ISO_8859_3_INIT), //IBM EBCDIC Latin America-Spain (20284 + Euro symbol); IBM EBCDIC (Spain-Euro)
		1146	=> Option::Some(&UTF_8_INIT), //IBM EBCDIC United Kingdom (20285 + Euro symbol); IBM EBCDIC (UK-Euro)
		1147	=> Option::Some(&WINDOWS_1252_INIT), //IBM EBCDIC France (20297 + Euro symbol); IBM EBCDIC (France-Euro)
		//1148	IBM01148	IBM EBCDIC International (500 + Euro symbol); IBM EBCDIC (International-Euro)
		1149	=> Option::Some(&WINDOWS_1257_INIT), //IBM EBCDIC Icelandic (20871 + Euro symbol); IBM EBCDIC (Icelandic-Euro)
		1200	=> Option::Some(&UTF_16LE_INIT), //Unicode UTF-16, little endian byte order (BMP of ISO 10646); available only to managed applications
		1201	=> Option::Some(&UTF_16BE_INIT), //Unicode UTF-16, big endian byte order; available only to managed applications
		1250	=> Option::Some(&WINDOWS_1250_INIT),// windows-1250	ANSI Central European; Central European (Windows)
		1251	=> Option::Some(&WINDOWS_1251_INIT),// windows-1251	ANSI Cyrillic; Cyrillic (Windows)
		1252	=> Option::Some(&WINDOWS_1252_INIT),// windows-1252	ANSI Latin 1; Western European (Windows)
		1253	=> Option::Some(&WINDOWS_1253_INIT),// windows-1253	ANSI Greek; Greek (Windows)
		1254	=> Option::Some(&WINDOWS_1254_INIT),// windows-1254	ANSI Turkish; Turkish (Windows)
		1255	=> Option::Some(&WINDOWS_1255_INIT),// windows-1255	ANSI Hebrew; Hebrew (Windows)
		1256	=> Option::Some(&WINDOWS_1256_INIT),// windows-1256	ANSI Arabic; Arabic (Windows)
		1257	=> Option::Some(&WINDOWS_1257_INIT),// windows-1257	ANSI Baltic; Baltic (Windows)
		1258	=> Option::Some(&WINDOWS_1258_INIT),// windows-1258	ANSI/OEM Vietnamese; Vietnamese (Windows)
		1361	=> Option::Some(&EUC_KR_INIT),      //Johab	Korean (Johab)
		10000	=> Option::Some(&WINDOWS_1252_INIT), //macintosh	MAC Roman; Western European (Mac)
		10001	=> Option::Some(&SHIFT_JIS_INIT),//x-mac-japanese	Japanese (Mac)
		10002	=> Option::Some(&GBK_INIT), //x-mac-chinesetrad	MAC Traditional Chinese (Big5); Chinese Traditional (Mac)
		10003	=> Option::Some(&EUC_KR_INIT),  //x-mac-korean	Korean (Mac)
		10004	=> Option::Some(&WINDOWS_1256_INIT), //x-mac-arabic	Arabic (Mac)
		10005	=> Option::Some(&ISO_8859_8_INIT),//x-mac-hebrew	Hebrew (Mac)
		10006	=> Option::Some(&ISO_8859_7_INIT), // x-mac-greek	Greek (Mac)
		10007	=> Option::Some(&X_MAC_CYRILLIC_INIT), //x-mac-cyrillic	Cyrillic (Mac)
		10008	=> Option::Some(&GBK_INIT), // x-mac-chinesesimp	MAC Simplified Chinese (GB 2312); Chinese Simplified (Mac)
		10010	=> Option::Some(&ISO_8859_2_INIT), // x-mac-romanian	Romanian (Mac)
		10017	=> Option::Some(&ISO_8859_5_INIT), //x-mac-ukrainian	Ukrainian (Mac)
		10021	=> Option::Some(&WINDOWS_874_INIT), // x-mac-thai	Thai (Mac)
		10029	=> Option::Some(&ISO_8859_2_INIT), //x-mac-ce	MAC Latin 2; Central European (Mac)
		10079	=> Option::Some(&WINDOWS_1257_INIT),//x-mac-icelandic	Icelandic (Mac)
		10081	=> Option::Some(&WINDOWS_1254_INIT),//x-mac-turkish	Turkish (Mac)
		//10082	x-mac-croatian	Croatian (Mac)
		//12000	=>&UTF_16LE utf-32	Unicode UTF-32, little endian byte order; available only to managed applications
		//12001	utf-32BE	Unicode UTF-32, big endian byte order; available only to managed applications
		20000	=> Option::Some(&GBK_INIT), //x-Chinese_CNS	CNS Taiwan; Chinese Traditional (CNS)
		20001	=> Option::Some(&GBK_INIT), //x-cp20001	TCA Taiwan
		20002	=> Option::Some(&GBK_INIT), //x_Chinese-Eten	Eten Taiwan; Chinese Traditional (Eten)
		20003	=> Option::Some(&GBK_INIT), //x-cp20003	IBM5550 Taiwan
		20004	=> Option::Some(&GBK_INIT), //x-cp20004	TeleText Taiwan
		20005	=> Option::Some(&GBK_INIT), //x-cp20005	Wang Taiwan
		20105	=> Option::Some(&WINDOWS_1252_INIT), // x-IA5	IA5 (IRV International Alphabet No. 5, 7-bit); Western European (IA5)
		20106	=> Option::Some(&WINDOWS_1252_INIT), // x-IA5-German	IA5 German (7-bit)
		20107	=> Option::Some(&ISO_8859_4_INIT), // x-IA5-Swedish	IA5 Swedish (7-bit)
		20108	=> Option::Some(&ISO_8859_4_INIT), //x-IA5-Norwegian	IA5 Norwegian (7-bit)
		20127	=> Option::Some(&UTF_8_INIT), // us-ascii	US-ASCII (7-bit)
		// 20261 x-cp20261	T.61
		//20269	x-cp20269	ISO 6937 Non-Spacing Accent
		20273   => Option::Some(&WINDOWS_1252_INIT), //	IBM273	IBM EBCDIC Germany
		20277	=> Option::Some(&ISO_8859_4_INIT), //IBM277	IBM EBCDIC Denmark-Norway
		20278	=> Option::Some(&ISO_8859_4_INIT), //IBM278	IBM EBCDIC Finland-Sweden
		20280	=> Option::Some(&ISO_8859_3_INIT),  //IBM280	IBM EBCDIC Italy
		20284	=> Option::Some(&ISO_8859_3_INIT), //IBM284	IBM EBCDIC Latin America-Spain
		20285	=> Option::Some(&UTF_8_INIT), // IBM285	IBM EBCDIC United Kingdom
		20290	=> Option::Some(&SHIFT_JIS_INIT), //IBM290	IBM EBCDIC Japanese Katakana Extended
		20297	=> Option::Some(&WINDOWS_1252_INIT),//IBM297	IBM EBCDIC France
		20420	=> Option::Some(&WINDOWS_1256_INIT),//IBM420	IBM EBCDIC Arabic
		20423	=> Option::Some(&ISO_8859_7_INIT), // IBM423	IBM EBCDIC Greek
		20424	=> Option::Some(&ISO_8859_8_INIT), // IBM424	IBM EBCDIC Hebrew
		20833   => Option::Some(&EUC_KR_INIT),//x-EBCDIC-KoreanExtended	IBM EBCDIC Korean Extended
		20838   => Option::Some(&WINDOWS_874_INIT), //	IBM-Thai	IBM EBCDIC Thai
		20866   => Option::Some(&ISO_8859_5_INIT), //koi8-r	Russian (KOI8-R); Cyrillic (KOI8-R)
		20871	=> Option::Some(&WINDOWS_1257_INIT), //IBM871	IBM EBCDIC Icelandic
		20880	=> Option::Some(&ISO_8859_5_INIT), //IBM880	IBM EBCDIC Cyrillic Russian
		20905	=> Option::Some(&WINDOWS_1254_INIT),//IBM905	IBM EBCDIC Turkish
		20924	=> Option::Some(&WINDOWS_1252_INIT),//IBM00924	IBM EBCDIC Latin 1/Open System (1047 + Euro symbol)
		20932   => Option::Some(&SHIFT_JIS_INIT),//EUC-JP	Japanese (JIS 0208-1990 and 0212-1990)
		20936   => Option::Some(&GBK_INIT),        //	x-cp20936	Simplified Chinese (GB2312); Chinese Simplified (GB2312-80)
		20949   => Option::Some(&EUC_KR_INIT), //x-cp20949	Korean Wansung
		21025	=> Option::Some(&ISO_8859_5_INIT),  //cp1025	IBM EBCDIC Cyrillic Serbian-Bulgarian
		//21027		(deprecated)
		21866	=> Option::Some(&ISO_8859_5_INIT), //koi8-u	Ukrainian (KOI8-U); Cyrillic (KOI8-U)
		28591	=> Option::Some(&WINDOWS_1252_INIT),// iso-8859-1	ISO 8859-1 Latin 1; Western European (ISO)
		28592	=> Option::Some(&ISO_8859_2_INIT), //iso-8859-2	ISO 8859-2 Central European; Central European (ISO)
		28593	=> Option::Some(&ISO_8859_3_INIT), //iso-8859-3	ISO 8859-3 Latin 3
		28594	=> Option::Some(&ISO_8859_4_INIT), //iso-8859-4	ISO 8859-4 Baltic
		28595	=> Option::Some(&ISO_8859_5_INIT), //iso-8859-5	ISO 8859-5 Cyrillic
		28596	=> Option::Some(&ISO_8859_6_INIT), //iso-8859-6	ISO 8859-6 Arabic
		28597	=> Option::Some(&ISO_8859_7_INIT), //iso-8859-7	ISO 8859-7 Greek
		28598	=> Option::Some(&ISO_8859_8_INIT), //iso-8859-8	ISO 8859-8 Hebrew; Hebrew (ISO-Visual)
		28599	=> Option::Some(&WINDOWS_1254_INIT), //iso-8859-9	ISO 8859-9 Turkish
		28603	=> Option::Some(&ISO_8859_13_INIT), //iso-8859-13	ISO 8859-13 Estonian
		28605	=> Option::Some(&ISO_8859_15_INIT), //iso-8859-15	ISO 8859-15 Latin 9
		29001	=> Option::Some(&ISO_8859_16_INIT), //x-Europa	Europa 3
		38598	=> Option::Some(&ISO_8859_8_INIT), //iso-8859-8-i	ISO 8859-8 Hebrew; Hebrew (ISO-Logical)
		50220	=> Option::Some(&SHIFT_JIS_INIT), //iso-2022-jp	ISO 2022 Japanese with no halfwidth Katakana; Japanese (JIS)
		50221	=> Option::Some(&SHIFT_JIS_INIT), //csISO2022JP	ISO 2022 Japanese with halfwidth Katakana; Japanese (JIS-Allow 1 byte Kana)
		50222	=> Option::Some(&SHIFT_JIS_INIT), //iso-2022-jp	ISO 2022 Japanese JIS X 0201-1989; Japanese (JIS-Allow 1 byte Kana - SO/SI)
		50225	=> Option::Some(&EUC_KR_INIT), //iso-2022-kr	ISO 2022 Korean
		50227	=> Option::Some(&GBK_INIT), //x-cp50227	ISO 2022 Simplified Chinese; Chinese Simplified (ISO 2022)
		50229	=> Option::Some(&GBK_INIT), 	//ISO 2022 Traditional Chinese
		50930	=> Option::Some(&SHIFT_JIS_INIT), //	EBCDIC Japanese (Katakana) Extended
		50931	=> Option::Some(&SHIFT_JIS_INIT), //	EBCDIC US-Canada and Japanese
		50933	=> Option::Some(&EUC_KR_INIT), //	EBCDIC Korean Extended and Korean
		50935	=> Option::Some(&GBK_INIT), 	//EBCDIC Simplified Chinese Extended and Simplified Chinese
		50936	=> Option::Some(&GBK_INIT), 	//EBCDIC Simplified Chinese
		50937	=> Option::Some(&GBK_INIT), 	//EBCDIC US-Canada and Traditional Chinese
		50939	=> Option::Some(&SHIFT_JIS_INIT),	//EBCDIC Japanese (Latin) Extended and Japanese
		51932	=> Option::Some(&SHIFT_JIS_INIT),	//euc-jp	EUC Japanese
		51936	=> Option::Some(&GBK_INIT),  //EUC-CN	EUC Simplified Chinese; Chinese Simplified (EUC)
		51949	=> Option::Some(&EUC_KR_INIT), //euc-kr	EUC Korean
		51950	=> Option::Some(&GBK_INIT),	//EUC Traditional Chinese
		52936	=> Option::Some(&GBK_INIT), //hz-gb-2312	HZ-GB2312 Simplified Chinese; Chinese Simplified (HZ)
		54936	=> Option::Some(&GB18030_INIT), //GB18030	Windows XP and later: GB18030 Simplified Chinese (4 byte); Chinese Simplified (GB18030)
		//57002   x-iscii-de	ISCII Devanagari
		//57003	x-iscii-be	ISCII Bangla
		//57004	x-iscii-ta	ISCII Tamil
		//57005	x-iscii-te	ISCII Telugu
		//57006	x-iscii-as	ISCII Assamese
		//57007	x-iscii-or	ISCII Odia
		//57008	x-iscii-ka	ISCII Kannada
		//57009	x-iscii-ma	ISCII Malayalam
		//57010	x-iscii-gu	ISCII Gujarati
		//57011	x-iscii-pa	ISCII Punjabi
		//65000   utf-7	Unicode (UTF-7)
		65001   => Option::Some(&UTF_8_INIT),	//utf-8	Unicode (UTF-8)
        _       => None
	}
}
