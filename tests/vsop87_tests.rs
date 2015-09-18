extern crate vsop87;
use vsop87::vsop87::*;

#[test]
fn it_mercury() {
    let (a, l, k, h, q, p) = mercury(2451545.0);

    assert!(a > 0.3870982121 && a < 0.3870982123);
    assert!(l > 4.4026057778 && l < 4.4026057780);
    assert!(k > 0.0446647517 && k < 0.0446647519);
    assert!(h > 0.2007208957 && h < 0.2007208959);
    assert!(q > 0.0406161540 && q < 0.0406161542);
    assert!(p > 0.04563512 && p < 0.04563588);

    let (a, l, k, h, q, p) = mercury(2415020.0);

    assert!(a > 0.3870977205 && a < 0.3870977207);
    assert!(l > 3.1341564064 && l < 3.1341564066);
    assert!(k > 0.0452159417 && k < 0.0452159419);
    assert!(h > 0.2005915793 && h < 0.2005915795);
    assert!(q > 0.0405500077 && q < 0.0405500079);
    assert!(p > 0.04576328 && p < 0.04576404);

    let (a, l, k, h, q, p) = mercury(2378495.0);

    assert!(a > 0.3870988717 && a < 0.3870988719);
    assert!(l > 1.8657954072 && l < 1.8657954074);
    assert!(k > 0.0457588297 && k < 0.0457588299);
    assert!(h > 0.2004369023 && h < 0.2004369025);
    assert!(q > 0.0404841230 && q < 0.0404841232);
    assert!(p > 0.04589017 && p < 0.04589093);

    let (a, l, k, h, q, p) = mercury(2341970.0);

    assert!(a > 0.3870981591 && a < 0.3870981593);
    assert!(l > 0.5973516889 && l < 0.5973516891);
    assert!(k > 0.0462989388 && k < 0.0462989390);
    assert!(h > 0.2002875698 && h < 0.2002875700);
    assert!(q > 0.0404178809 && q < 0.0404178811);
    assert!(p > 0.04601680 && p < 0.04601756);

    let (a, l, k, h, q, p) = mercury(2305445.0);

    assert!(a > 0.3870993314 && a < 0.3870993316);
    assert!(l > 5.6121176788 && l < 5.6121176790);
    assert!(k > 0.0468597369 && k < 0.0468597371);
    assert!(h > 0.2001268145 && h < 0.2001268147);
    assert!(q > 0.0403519525 && q < 0.0403519527);
    assert!(p > 0.04614370 && p < 0.04614446);

    let (a, l, k, h, q, p) = mercury(2268920.0);

    assert!(a > 0.3870987714 && a < 0.3870987716);
    assert!(l > 4.3437933703 && l < 4.3437933705);
    assert!(k > 0.0474175873 && k < 0.0474175875);
    assert!(h > 0.1999792559 && h < 0.1999792561);
    assert!(q > 0.0402856290 && q < 0.0402856292);
    assert!(p > 0.04627099 && p < 0.04627175);

    let (a, l, k, h, q, p) = mercury(2232395.0);

    assert!(a > 0.3870987732 && a < 0.3870987734);
    assert!(l > 3.0753253704 && l < 3.0753253706);
    assert!(k > 0.0479739902 && k < 0.0479739904);
    assert!(h > 0.1998210216 && h < 0.1998210218);
    assert!(q > 0.0402194430 && q < 0.0402194432);
    assert!(p > 0.04639750 && p < 0.04639826);

    let (a, l, k, h, q, p) = mercury(2195870.0);

    assert!(a > 0.3870975282 && a < 0.3870975284);
    assert!(l > 1.8068957209 && l < 1.8068957211);
    assert!(k > 0.0485220074 && k < 0.0485220076);
    assert!(h > 0.1996800562 && h < 0.1996800564);
    assert!(q > 0.0401524549 && q < 0.0401524551);
    assert!(p > 0.04652410 && p < 0.04652486);

    let (a, l, k, h, q, p) = mercury(2159345.0);

    assert!(a > 0.3870977788 && a < 0.3870977790);
    assert!(l > 0.5385405029 && l < 0.5385405031);
    assert!(k > 0.0490792961 && k < 0.0490792963);
    assert!(h > 0.1995293399 && h < 0.1995293401);
    assert!(q > 0.0400854728 && q < 0.0400854730);
    assert!(p > 0.04665108 && p < 0.04665184);

    let (a, l, k, h, q, p) = mercury(2122820.0);

    assert!(a > 0.3870983086 && a < 0.3870983088);
    assert!(l > 5.5532501233 && l < 5.5532501235);
    assert!(k > 0.0496101938 && k < 0.0496101940);
    assert!(h > 0.1993701566 && h < 0.1993701568);
    assert!(q > 0.0400177911 && q < 0.0400177913);
    assert!(p > 0.04677625 && p < 0.04677701);
}

#[test]
fn it_venus() {
    let (a, l, k, h, q, p) = venus(2451545.0);

    assert!(a > 0.7233269303 && a < 0.7233269305);
    assert!(l > 3.1761350909 && l < 3.1761350911);
    assert!(k > -0.0045086078 && k < -0.0045086076);
    assert!(h > 0.0050312181 && h < 0.0050312183);
    assert!(q > 0.0068248057 && q < 0.0068248059);
    assert!(p > 0.02882177 && p < 0.02882253);

    let (a, l, k, h, q, p) = venus(2415020.0);

    assert!(a > 0.7233254386 && a < 0.7233254388);
    assert!(l > 6.0067809875 && l < 6.0067809877);
    assert!(k > -0.0044945273 && k < -0.0044945271);
    assert!(h > 0.0051121732 && h < 0.0051121734);
    assert!(q > 0.0066855873 && q < 0.0066855875);
    assert!(p > 0.02886332 && p < 0.02886408);

    let (a, l, k, h, q, p) = venus(2378495.0);

    assert!(a > 0.7233259623 && a < 0.7233259625);
    assert!(l > 2.5541924032 && l < 2.5541924034);
    assert!(k > -0.0045347114 && k < -0.0045347112);
    assert!(h > 0.0051153685 && h < 0.0051153687);
    assert!(q > 0.0065487272 && q < 0.0065487274);
    assert!(p > 0.02890118 && p < 0.02890194);

    let (a, l, k, h, q, p) = venus(2341970.0);

    assert!(a > 0.7233336009 && a < 0.7233336011);
    assert!(l > 5.3848916849 && l < 5.3848916851);
    assert!(k > -0.0045714047 && k < -0.0045714045);
    assert!(h > 0.0052085665 && h < 0.0052085667);
    assert!(q > 0.0064078361 && q < 0.0064078363);
    assert!(p > 0.02893776 && p < 0.02893852);

    let (a, l, k, h, q, p) = venus(2305445.0);

    assert!(a > 0.7233444127 && a < 0.7233444129);
    assert!(l > 1.9323601819 && l < 1.9323601821);
    assert!(k > -0.0046106822 && k < -0.0046106820);
    assert!(h > 0.0052198094 && h < 0.0052198096);
    assert!(q > 0.0062695295 && q < 0.0062695297);
    assert!(p > 0.02897438 && p < 0.02897514);

    let (a, l, k, h, q, p) = venus(2268920.0);

    assert!(a > 0.7233295420 && a < 0.7233295422);
    assert!(l > 4.7630365986 && l < 4.7630365988);
    assert!(k > -0.0046705687 && k < -0.0046705685);
    assert!(h > 0.0052905837 && h < 0.0052905839);
    assert!(q > 0.0061312555 && q < 0.0061312557);
    assert!(p > 0.02900830 && p < 0.02900906);

    let (a, l, k, h, q, p) = venus(2232395.0);

    assert!(a > 0.7233288476 && a < 0.7233288478);
    assert!(l > 1.3105445954 && l < 1.3105445956);
    assert!(k > -0.0046816593 && k < -0.0046816591);
    assert!(h > 0.0052685213 && h < 0.0052685215);
    assert!(q > 0.0059904847 && q < 0.0059904849);
    assert!(p > 0.02904208 && p < 0.02904284);

    let (a, l, k, h, q, p) = venus(2195870.0);

    assert!(a > 0.7233301131 && a < 0.7233301133);
    assert!(l > 4.1411623838 && l < 4.1411623840);
    assert!(k > -0.0047512384 && k < -0.0047512382);
    assert!(h > 0.0053213978 && h < 0.0053213980);
    assert!(q > 0.0058529531 && q < 0.0058529533);
    assert!(p > 0.02907519 && p < 0.02907595);

    let (a, l, k, h, q, p) = venus(2159345.0);

    assert!(a > 0.7233280697 && a < 0.7233280699);
    assert!(l > 0.6886289924 && l < 0.6886289926);
    assert!(k > -0.0047241640 && k < -0.0047241638);
    assert!(h > 0.0053523285 && h < 0.0053523287);
    assert!(q > 0.0057132256 && q < 0.0057132258);
    assert!(p > 0.02910527 && p < 0.02910603);

    let (a, l, k, h, q, p) = venus(2122820.0);

    assert!(a > 0.7233247250 && a < 0.7233247252);
    assert!(l > 3.5192700748 && l < 3.5192700750);
    assert!(k > -0.0047739163 && k < -0.0047739161);
    assert!(h > 0.0053755161 && h < 0.0053755163);
    assert!(q > 0.0055732703 && q < 0.0055732705);
    assert!(p > 0.02913516 && p < 0.02913592);
}

#[test]
fn it_mars() {
    let (a, l, k, h, q, p) = mars(2451545.0);

    assert!(a > 1.5236789886 && a < 1.5236789888);
    assert!(l > 6.2038757098 && l < 6.2038757100);
    assert!(k > 0.0853133077 && k < 0.0853133079);
    assert!(h > -0.0378067118 && h < -0.0378067116);
    assert!(q > 0.0104705228 && q < 0.0104705230);
    assert!(p > 0.01228588 && p < 0.01228664);

    let (a, l, k, h, q, p) = mars(2415020.0);

    assert!(a > 1.5236463471 && a < 1.5236463473);
    assert!(l > 5.1511909416 && l < 5.1511909418);
    assert!(k > 0.0849023066 && k < 0.0849023068);
    assert!(h > -0.0384449186 && h < -0.0384449184);
    assert!(q > 0.0104523542 && q < 0.0104523544);
    assert!(p > 0.01239228 && p < 0.01239304);

    let (a, l, k, h, q, p) = mars(2378495.0);

    assert!(a > 1.5236769794 && a < 1.5236769796);
    assert!(l > 4.0987790796 && l < 4.0987790798);
    assert!(k > 0.0846457411 && k < 0.0846457413);
    assert!(h > -0.0392454968 && h < -0.0392454966);
    assert!(q > 0.0104339357 && q < 0.0104339359);
    assert!(p > 0.01249909 && p < 0.01249985);

    let (a, l, k, h, q, p) = mars(2341970.0);

    assert!(a > 1.5236301763 && a < 1.5236301765);
    assert!(l > 3.0462792684 && l < 3.0462792686);
    assert!(k > 0.0842278892 && k < 0.0842278894);
    assert!(h > -0.0396982630 && h < -0.0396982628);
    assert!(q > 0.0104148074 && q < 0.0104148076);
    assert!(p > 0.01260671 && p < 0.01260747);

    let (a, l, k, h, q, p) = mars(2305445.0);

    assert!(a > 1.5237744504 && a < 1.5237744506);
    assert!(l > 1.9938502930 && l < 1.9938502932);
    assert!(k > 0.0837767412 && k < 0.0837767414);
    assert!(h > -0.0403804192 && h < -0.0403804190);
    assert!(q > 0.0103948926 && q < 0.0103948928);
    assert!(p > 0.01271228 && p < 0.01271304);

    let (a, l, k, h, q, p) = mars(2268920.0);

    assert!(a > 1.5236102236 && a < 1.5236102238);
    assert!(l > 0.9412449946 && l < 0.9412449948);
    assert!(k > 0.0833757043 && k < 0.0833757045);
    assert!(h > -0.0410377667 && h < -0.0410377665);
    assert!(q > 0.0103755321 && q < 0.0103755323);
    assert!(p > 0.01281888 && p < 0.01281963);

    let (a, l, k, h, q, p) = mars(2232395.0);

    assert!(a > 1.5237000650 && a < 1.5237000652);
    assert!(l > 6.1719713982 && l < 6.1719713984);
    assert!(k > 0.0831244747 && k < 0.0831244749);
    assert!(h > -0.0415495115 && h < -0.0415495113);
    assert!(q > 0.0103538428 && q < 0.0103538430);
    assert!(p > 0.01292515 && p < 0.01292591);

    let (a, l, k, h, q, p) = mars(2195870.0);

    assert!(a > 1.5236148014 && a < 1.5236148016);
    assert!(l > 5.1194225687 && l < 5.1194225689);
    assert!(k > 0.0825058310 && k < 0.0825058312);
    assert!(h > -0.0421055437 && h < -0.0421055435);
    assert!(q > 0.0103313654 && q < 0.0103313656);
    assert!(p > 0.01303122 && p < 0.01303198);

    let (a, l, k, h, q, p) = mars(2159345.0);

    assert!(a > 1.5237578877 && a < 1.5237578879);
    assert!(l > 4.0669853278 && l < 4.0669853280);
    assert!(k > 0.0821906316 && k < 0.0821906318);
    assert!(h > -0.0427917583 && h < -0.0427917581);
    assert!(q > 0.0103081045 && q < 0.0103081047);
    assert!(p > 0.01313608 && p < 0.01313684);

    let (a, l, k, h, q, p) = mars(2122820.0);

    assert!(a > 1.5236045524 && a < 1.5236045526);
    assert!(l > 3.0145860937 && l < 3.0145860939);
    assert!(k > 0.0818247407 && k < 0.0818247409);
    assert!(h > -0.0434649475 && h < -0.0434649473);
    assert!(q > 0.0102833000 && q < 0.0102833002);
    assert!(p > 0.01324009 && p < 0.01324085);
}

#[test]
fn it_jupiter() {
    let (a, l, k, h, q, p) = jupiter(2451545.0);

    assert!(a > 5.2042662907 && a < 5.2042662909);
    assert!(l > 0.5999772954 && l < 0.5999772956);
    assert!(k > 0.0469877115 && k < 0.0469877117);
    assert!(h > 0.0130817657 && h < 0.0130817659);
    assert!(q > -0.0020729463 && q < -0.0020729461);
    assert!(p > 0.01119395 && p < 0.01119471);

    let (a, l, k, h, q, p) = jupiter(2415020.0);

    assert!(a > 5.2028202640 && a < 5.2028202642);
    assert!(l > 4.1841549083 && l < 4.1841549085);
    assert!(k > 0.0473151972 && k < 0.0473151974);
    assert!(h > 0.0115865095 && h < 0.0115865097);
    assert!(q > -0.0020327889 && q < -0.0020327887);
    assert!(p > 0.01121514 && p < 0.01121590);

    let (a, l, k, h, q, p) = jupiter(2378495.0);

    assert!(a > 5.2027276672 && a < 5.2027276674);
    assert!(l > 1.4820596291 && l < 1.4820596293);
    assert!(k > 0.0464780412 && k < 0.0464780414);
    assert!(h > 0.0116460263 && h < 0.0116460265);
    assert!(q > -0.0019921307 && q < -0.0019921305);
    assert!(p > 0.01123447 && p < 0.01123523);

    let (a, l, k, h, q, p) = jupiter(2341970.0);

    assert!(a > 5.2019341868 && a < 5.2019341870);
    assert!(l > 5.0599431121 && l < 5.0599431123);
    assert!(k > 0.0475018339 && k < 0.0475018341);
    assert!(h > 0.0119836367 && h < 0.0119836369);
    assert!(q > -0.0019621322 && q < -0.0019621320);
    assert!(p > 0.01125575 && p < 0.01125651);

    let (a, l, k, h, q, p) = jupiter(2305445.0);

    assert!(a > 5.2018720769 && a < 5.2018720771);
    assert!(l > 2.3540228341 && l < 2.3540228343);
    assert!(k > 0.0468736015 && k < 0.0468736017);
    assert!(h > 0.0103577795 && h < 0.0103577797);
    assert!(q > -0.0019360509 && q < -0.0019360507);
    assert!(p > 0.01127707 && p < 0.01127783);

    let (a, l, k, h, q, p) = jupiter(2268920.0);

    assert!(a > 5.2018353755 && a < 5.2018353757);
    assert!(l > 5.9291206443 && l < 5.9291206445);
    assert!(k > 0.0457181531 && k < 0.0457181533);
    assert!(h > 0.0107163503 && h < 0.0107163505);
    assert!(q > -0.0019130012 && q < -0.0019130010);
    assert!(p > 0.01129724 && p < 0.01129800);

    let (a, l, k, h, q, p) = jupiter(2232395.0);

    assert!(a > 5.2016905938 && a < 5.2016905940);
    assert!(l > 3.2222446781 && l < 3.2222446783);
    assert!(k > 0.0468278448 && k < 0.0468278450);
    assert!(h > 0.0106167943 && h < 0.0106167945);
    assert!(q > -0.0018866373 && q < -0.0018866371);
    assert!(p > 0.01132280 && p < 0.01132356);

    let (a, l, k, h, q, p) = jupiter(2195870.0);

    assert!(a > 5.2022541042 && a < 5.2022541044);
    assert!(l > 0.5183166214 && l < 0.5183166216);
    assert!(k > 0.0456561945 && k < 0.0456561947);
    assert!(h > 0.0099030073 && h < 0.0099030075);
    assert!(q > -0.0018595889 && q < -0.0018595887);
    assert!(p > 0.01135867 && p < 0.01135943);

    let (a, l, k, h, q, p) = jupiter(2159345.0);

    assert!(a > 5.2025950507 && a < 5.2025950509);
    assert!(l > 4.0986003153 && l < 4.0986003155);
    assert!(k > 0.0451688311 && k < 0.0451688313);
    assert!(h > 0.0110668122 && h < 0.0110668124);
    assert!(q > -0.0018350865 && q < -0.0018350863);
    assert!(p > 0.01139106 && p < 0.01139182);

    let (a, l, k, h, q, p) = jupiter(2122820.0);

    assert!(a > 5.2026637692 && a < 5.2026637694);
    assert!(l > 1.3989872238 && l < 1.3989872240);
    assert!(k > 0.0462126898 && k < 0.0462126900);
    assert!(h > 0.0102292159 && h < 0.0102292161);
    assert!(q > -0.0018026305 && q < -0.0018026303);
    assert!(p > 0.01141772 && p < 0.01141848);
}

#[test]
fn it_saturn() {
    let (a, l, k, h, q, p) = saturn(2451545.0);

    assert!(a > 9.5820161866 && a < 9.5820161868);
    assert!(l > 0.8727430949 && l < 0.8727430951);
    assert!(k > 0.0003336008 && k < 0.0003336010);
    assert!(h > 0.0557224685 && h < 0.0557224687);
    assert!(q > -0.0086968780 && q < -0.0086968778);
    assert!(p > 0.01986563 && p < 0.01986639);

    let (a, l, k, h, q, p) = saturn(2415020.0);

    assert!(a > 9.5797975825 && a < 9.5797975827);
    assert!(l > 4.6635485633 && l < 4.6635485635);
    assert!(k > -0.0037561571 && k < -0.0037561569);
    assert!(h > 0.0510499910 && h < 0.0510499912);
    assert!(q > -0.0087942465 && q < -0.0087942463);
    assert!(p > 0.01980654 && p < 0.01980730);

    let (a, l, k, h, q, p) = saturn(2378495.0);

    assert!(a > 9.5845294674 && a < 9.5845294676);
    assert!(l > 2.1792108199 && l < 2.1792108201);
    assert!(k > -0.0040674040 && k < -0.0040674038);
    assert!(h > 0.0594660901 && h < 0.0594660903);
    assert!(q > -0.0088957952 && q < -0.0088957950);
    assert!(p > 0.01975409 && p < 0.01975485);

    let (a, l, k, h, q, p) = saturn(2341970.0);

    assert!(a > 9.5793512112 && a < 9.5793512114);
    assert!(l > 5.9857401998 && l < 5.9857402000);
    assert!(k > -0.0007610587 && k < -0.0007610585);
    assert!(h > 0.0541118731 && h < 0.0541118733);
    assert!(q > -0.0089677722 && q < -0.0089677720);
    assert!(p > 0.01970184 && p < 0.01970260);

    let (a, l, k, h, q, p) = saturn(2305445.0);

    assert!(a > 9.5727100002 && a < 9.5727100004);
    assert!(l > 3.5107821038 && l < 3.5107821040);
    assert!(k > -0.0048218813 && k < -0.0048218811);
    assert!(h > 0.0575514202 && h < 0.0575514204);
    assert!(q > -0.0090348990 && q < -0.0090348988);
    assert!(p > 0.01965756 && p < 0.01965832);

    let (a, l, k, h, q, p) = saturn(2268920.0);

    assert!(a > 9.5665592834 && a < 9.5665592836);
    assert!(l > 1.0414908681 && l < 1.0414908683);
    assert!(k > 0.0023388087 && k < 0.0023388089);
    assert!(h > 0.0601498959 && h < 0.0601498961);
    assert!(q > -0.0090989758 && q < -0.0090989756);
    assert!(p > 0.01961148 && p < 0.01961224);

    let (a, l, k, h, q, p) = saturn(2232395.0);

    assert!(a > 9.5588545246 && a < 9.5588545248);
    assert!(l > 4.8521539279 && l < 4.8521539281);
    assert!(k > -0.0000063021 && k < -0.0000063019);
    assert!(h > 0.0578024218 && h < 0.0578024220);
    assert!(q > -0.0091690231 && q < -0.0091690229);
    assert!(p > 0.01954496 && p < 0.01954572);

    let (a, l, k, h, q, p) = saturn(2195870.0);

    assert!(a > 9.5448204346 && a < 9.5448204348);
    assert!(l > 2.3727005008 && l < 2.3727005010);
    assert!(k > 0.0022420512 && k < 0.0022420514);
    assert!(h > 0.0594072383 && h < 0.0594072385);
    assert!(q > -0.0092439695 && q < -0.0092439693);
    assert!(p > 0.01945077 && p < 0.01945153);

    let (a, l, k, h, q, p) = saturn(2159345.0);

    assert!(a > 9.5363776410 && a < 9.5363776412);
    assert!(l > 6.1737761791 && l < 6.1737761793);
    assert!(k > 0.0034673265 && k < 0.0034673267);
    assert!(h > 0.0565365311 && h < 0.0565365313);
    assert!(q > -0.0093073645 && q < -0.0093073643);
    assert!(p > 0.01936955 && p < 0.01937031);

    let (a, l, k, h, q, p) = saturn(2122820.0);

    assert!(a > 9.5316426699 && a < 9.5316426701);
    assert!(l > 3.6837162324 && l < 3.6837162326);
    assert!(k > 0.0034527066 && k < 0.0034527068);
    assert!(h > 0.0581970244 && h < 0.0581970246);
    assert!(q > -0.0093872673 && q < -0.0093872671);
    assert!(p > 0.01930558 && p < 0.01930634);
}

#[test]
fn it_uranus() {
    let (a, l, k, h, q, p) = uranus(2451545.0);

    assert!(a > 19.2294229490 && a < 19.2294229492);
    assert!(l > 5.4713756705 && l < 5.4713756707);
    assert!(k > -0.0438022684 && k < -0.0438022682);
    assert!(h > 0.0073046994 && h < 0.0073046996);
    assert!(q > 0.0018594790 && q < 0.0018594792);
    assert!(p > 0.00648001 && p < 0.00648077);

    let (a, l, k, h, q, p) = uranus(2415020.0);

    assert!(a > 19.3136226839 && a < 19.3136226841);
    assert!(l > 4.2716633732 && l < 4.2716633734);
    assert!(k > -0.0488881225 && k < -0.0488881223);
    assert!(h > 0.0023646937 && h < 0.0023646939);
    assert!(q > 0.0018638195 && q < 0.0018638197);
    assert!(p > 0.00650629 && p < 0.00650705);

    let (a, l, k, h, q, p) = uranus(2378495.0);

    assert!(a > 19.2386333587 && a < 19.2386333589);
    assert!(l > 3.0730118894 && l < 3.0730118896);
    assert!(k > -0.0483021691 && k < -0.0483021689);
    assert!(h > 0.0101661419 && h < 0.0101661421);
    assert!(q > 0.0018715045 && q < 0.0018715047);
    assert!(p > 0.00652151 && p < 0.00652227);

    let (a, l, k, h, q, p) = uranus(2341970.0);

    assert!(a > 19.1230558110 && a < 19.1230558112);
    assert!(l > 1.8812287778 && l < 1.8812287780);
    assert!(k > -0.0447817248 && k < -0.0447817246);
    assert!(h > 0.0053751773 && h < 0.0053751775);
    assert!(q > 0.0019049676 && q < 0.0019049678);
    assert!(p > 0.00651013 && p < 0.00651089);

    let (a, l, k, h, q, p) = uranus(2305445.0);

    assert!(a > 19.1822072678 && a < 19.1822072680);
    assert!(l > 0.6902152336 && l < 0.6902152338);
    assert!(k > -0.0464228380 && k < -0.0464228378);
    assert!(h > 0.0091244199 && h < 0.0091244201);
    assert!(q > 0.0019242447 && q < 0.0019242449);
    assert!(p > 0.00654129 && p < 0.00654205);

    let (a, l, k, h, q, p) = uranus(2268920.0);

    assert!(a > 19.2962467890 && a < 19.2962467892);
    assert!(l > 5.7758134684 && l < 5.7758134686);
    assert!(k > -0.0401255855 && k < -0.0401255853);
    assert!(h > 0.0056751507 && h < 0.0056751509);
    assert!(q > 0.0019109734 && q < 0.0019109736);
    assert!(p > 0.00654797 && p < 0.00654873);

    let (a, l, k, h, q, p) = uranus(2232395.0);

    assert!(a > 19.2497356422 && a < 19.2497356424);
    assert!(l > 4.5777275752 && l < 4.5777275754);
    assert!(k > -0.0466529112 && k < -0.0466529110);
    assert!(h > 0.0051308956 && h < 0.0051308958);
    assert!(q > 0.0019206656 && q < 0.0019206658);
    assert!(p > 0.00655819 && p < 0.00655895);

    let (a, l, k, h, q, p) = uranus(2195870.0);

    assert!(a > 19.1545703279 && a < 19.1545703281);
    assert!(l > 3.3858021155 && l < 3.3858021157);
    assert!(k > -0.0434958030 && k < -0.0434958028);
    assert!(h > 0.0088974145 && h < 0.0088974147);
    assert!(q > 0.0019372306 && q < 0.0019372308);
    assert!(p > 0.00658337 && p < 0.00658413);

    let (a, l, k, h, q, p) = uranus(2159345.0);

    assert!(a > 19.1811347111 && a < 19.1811347113);
    assert!(l > 2.1967520045 && l < 2.1967520047);
    assert!(k > -0.0456007263 && k < -0.0456007261);
    assert!(h > 0.0068443676 && h < 0.0068443678);
    assert!(q > 0.0019640435 && q < 0.0019640437);
    assert!(p > 0.00658836 && p < 0.00658912);

    let (a, l, k, h, q, p) = uranus(2122820.0);

    assert!(a > 19.2685452352 && a < 19.2685452354);
    assert!(l > 1.0026901572 && l < 1.0026901574);
    assert!(k > -0.0428389229 && k < -0.0428389227);
    assert!(h > 0.0113976085 && h < 0.0113976087);
    assert!(q > 0.0019824674 && q < 0.0019824676);
    assert!(p > 0.00660903 && p < 0.00660979);
}

#[test]
fn it_neptune() {
    let (a, l, k, h, q, p) = neptune(2451545.0);

    assert!(a > 30.1036169705 && a < 30.1036169707);
    assert!(l > 5.3268987908 && l < 5.3268987910);
    assert!(k > 0.0089053320 && k < 0.0089053322);
    assert!(h > 0.0068181683 && h < 0.0068181685);
    assert!(q > -0.0102818995 && q < -0.0102818993);
    assert!(p > 0.01150175 && p < 0.01150251);

    let (a, l, k, h, q, p) = neptune(2415020.0);

    assert!(a > 29.9473727789 && a < 29.9473727791);
    assert!(l > 1.5103401220 && l < 1.5103401222);
    assert!(k > 0.0048403714 && k < 0.0048403716);
    assert!(h > 0.0022828947 && h < 0.0022828949);
    assert!(q > -0.0102936855 && q < -0.0102936853);
    assert!(p > 0.01150022 && p < 0.01150098);

    let (a, l, k, h, q, p) = neptune(2378495.0);

    assert!(a > 29.9925541926 && a < 29.9925541928);
    assert!(l > 3.9743817918 && l < 3.9743817920);
    assert!(k > 0.0060997431 && k < 0.0060997433);
    assert!(h > 0.0092474382 && h < 0.0092474384);
    assert!(q > -0.0102935575 && q < -0.0102935573);
    assert!(p > 0.01150802 && p < 0.01150878);

    let (a, l, k, h, q, p) = neptune(2341970.0);

    assert!(a > 30.1627820094 && a < 30.1627820096);
    assert!(l > 0.1605390710 && l < 0.1605390712);
    assert!(k > 0.0091870580 && k < 0.0091870582);
    assert!(h > 0.0043333831 && h < 0.0043333833);
    assert!(q > -0.0102857259 && q < -0.0102857257);
    assert!(p > 0.01151010 && p < 0.01151086);

    let (a, l, k, h, q, p) = neptune(2305445.0);

    assert!(a > 30.2702161622 && a < 30.2702161624);
    assert!(l > 2.6344819843 && l < 2.6344819845);
    assert!(k > 0.0001266623 && k < 0.0001266625);
    assert!(h > 0.0095018713 && h < 0.0095018715);
    assert!(q > -0.0102752821 && q < -0.0102752819);
    assert!(p > 0.01149703 && p < 0.01149779);

    let (a, l, k, h, q, p) = neptune(2268920.0);

    assert!(a > 30.1963044187 && a < 30.1963044189);
    assert!(l > 5.1088676118 && l < 5.1088676120);
    assert!(k > 0.0091964091 && k < 0.0091964093);
    assert!(h > 0.0031103619 && h < 0.0031103621);
    assert!(q > -0.0102800265 && q < -0.0102800263);
    assert!(p > 0.01148076 && p < 0.01148152);

    let (a, l, k, h, q, p) = neptune(2232395.0);

    assert!(a > 30.0205469235 && a < 30.0205469237);
    assert!(l > 1.2942368464 && l < 1.2942368466);
    assert!(k > 0.0036280450 && k < 0.0036280452);
    assert!(h > 0.0054820844 && h < 0.0054820846);
    assert!(q > -0.0102966028 && q < -0.0102966026);
    assert!(p > 0.01147638 && p < 0.01147714);

    let (a, l, k, h, q, p) = neptune(2195870.0);

    assert!(a > 29.9660361001 && a < 29.9660361003);
    assert!(l > 3.7591524119 && l < 3.7591524121);
    assert!(k > 0.0080747908 && k < 0.0080747910);
    assert!(h > 0.0084977048 && h < 0.0084977050);
    assert!(q > -0.0103068591 && q < -0.0103068589);
    assert!(p > 0.01148319 && p < 0.01148395);

    let (a, l, k, h, q, p) = neptune(2159345.0);

    assert!(a > 30.0586108829 && a < 30.0586108831);
    assert!(l > 6.2253193739 && l < 6.2253193741);
    assert!(k > 0.0053615686 && k < 0.0053615688);
    assert!(h > 0.0046522989 && h < 0.0046522991);
    assert!(q > -0.0103020951 && q < -0.0103020949);
    assert!(p > 0.01149464 && p < 0.01149540);

    let (a, l, k, h, q, p) = neptune(2122820.0);

    assert!(a > 30.2002490333 && a < 30.2002490335);
    assert!(l > 2.4115747748 && l < 2.4115747750);
    assert!(k > 0.0032261109 && k < 0.0032261111);
    assert!(h > 0.0104510183 && h < 0.0104510185);
    assert!(q > -0.0102898481 && q < -0.0102898479);
    assert!(p > 0.01149039 && p < 0.01149115);
}

#[test]
fn it_earth_moon() {
    let (a, l, k, h, q, p) = earth_moon(2451545.0);

    assert!(a > 0.9999964221 && a < 0.9999964223);
    assert!(l > 1.7534128815 && l < 1.7534128817);
    assert!(k > -0.0037339067 && k < -0.0037339065);
    assert!(h > 0.0162796345 && h < 0.0162796347);
    assert!(q > -0.0000006037 && q < -0.0000006035);
    assert!(p > 0.00000025 && p < 0.00000101);

    let (a, l, k, h, q, p) = earth_moon(2415020.0);

    assert!(a > 0.9999996844 && a < 0.9999996846);
    assert!(l > 1.7643937667 && l < 1.7643937669);
    assert!(k > -0.0036507092 && k < -0.0036507090);
    assert!(h > 0.0163633889 && h < 0.0163633891);
    assert!(q > 0.0001135348 && q < 0.0001135350);
    assert!(p > -0.00001064 && p < -0.00000988);

    let (a, l, k, h, q, p) = earth_moon(2378495.0);

    assert!(a > 1.0000244437 && a < 1.0000244439);
    assert!(l > 1.7753590238 && l < 1.7753590240);
    assert!(k > -0.0035970436 && k < -0.0035970434);
    assert!(h > 0.0164283611 && h < 0.0164283613);
    assert!(q > 0.0002263287 && q < 0.0002263289);
    assert!(p > -0.00001932 && p < -0.00001856);

    let (a, l, k, h, q, p) = earth_moon(2341970.0);

    assert!(a > 0.9999991295 && a < 0.9999991297);
    assert!(l > 1.7863136093 && l < 1.7863136095);
    assert!(k > -0.0035112303 && k < -0.0035112301);
    assert!(h > 0.0164802094 && h < 0.0164802096);
    assert!(q > 0.0003422697 && q < 0.0003422699);
    assert!(p > -0.00002679 && p < -0.00002603);

    let (a, l, k, h, q, p) = earth_moon(2305445.0);

    assert!(a > 0.9999995277 && a < 0.9999995279);
    assert!(l > 1.7972736109 && l < 1.7972736111);
    assert!(k > -0.0034204916 && k < -0.0034204914);
    assert!(h > 0.0165004096 && h < 0.0165004098);
    assert!(q > 0.0004561548 && q < 0.0004561550);
    assert!(p > -0.00003406 && p < -0.00003330);

    let (a, l, k, h, q, p) = earth_moon(2268920.0);

    assert!(a > 0.9999866241 && a < 0.9999866243);
    assert!(l > 1.8082350760 && l < 1.8082350762);
    assert!(k > -0.0033449833 && k < -0.0033449831);
    assert!(h > 0.0165522472 && h < 0.0165522474);
    assert!(q > 0.0005701580 && q < 0.0005701582);
    assert!(p > -0.00003926 && p < -0.00003850);

    let (a, l, k, h, q, p) = earth_moon(2232395.0);

    assert!(a > 0.9999994546 && a < 0.9999994548);
    assert!(l > 1.8191248026 && l < 1.8191248028);
    assert!(k > -0.0032035188 && k < -0.0032035186);
    assert!(h > 0.0166424875 && h < 0.0166424877);
    assert!(q > 0.0006860762 && q < 0.0006860764);
    assert!(p > -0.00004422 && p < -0.00004346);

    let (a, l, k, h, q, p) = earth_moon(2195870.0);

    assert!(a > 0.9999859332 && a < 0.9999859334);
    assert!(l > 1.8300786646 && l < 1.8300786648);
    assert!(k > -0.0031249208 && k < -0.0031249206);
    assert!(h > 0.0166697591 && h < 0.0166697593);
    assert!(q > 0.0007994894 && q < 0.0007994896);
    assert!(p > -0.00004884 && p < -0.00004808);

    let (a, l, k, h, q, p) = earth_moon(2159345.0);

    assert!(a > 1.0000057989 && a < 1.0000057991);
    assert!(l > 1.8410570603 && l < 1.8410570605);
    assert!(k > -0.0030431505 && k < -0.0030431503);
    assert!(h > 0.0167901446 && h < 0.0167901448);
    assert!(q > 0.0009147432 && q < 0.0009147434);
    assert!(p > -0.00005091 && p < -0.00005015);

    let (a, l, k, h, q, p) = earth_moon(2122820.0);

    assert!(a > 1.0000134925 && a < 1.0000134927);
    assert!(l > 1.8519621672 && l < 1.8519621674);
    assert!(k > -0.0029638176 && k < -0.0029638174);
    assert!(h > 0.0168402193 && h < 0.0168402195);
    assert!(q > 0.0010301900 && q < 0.0010301902);
    assert!(p > -0.00005346 && p < -0.00005270);
}