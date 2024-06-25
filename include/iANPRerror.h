/******************************************************************************
	(C) 2018. IntBuSoft. http://intbusoft.com
	iANPRerror.h - version 1.7
*******************************************************************************/
#pragma once

#define IMAGE_EMPTY						-2	  // Изображение пустое
#define ERROR_RECT						-1	  // Ошибка в размерах области на изображении
#define ERROR_SIZE_XML_BUF				-3	  // Недостаточный размер XML-буфера
#define ERROR_TYPE_PLATE				-100  // Неподдерживаемый для данной конфигурации тип номера
#define ERROR_TYPE_FOR_COLOR			-101  // Неподдерживаемый флаг для цветного изображения - только ANPR_RUSSIAN_EXTENDED2 и ANPR_RUSSIAN_EXTENDED3
#define ERROR_NO_CUSTOM_DATA			-102  // Нет данных для настраиваемого типа данных

#define CAPTURE_EMPTY					-1000 // Пустой поток iANPRCapture
#define ERROR_CREATEMEMORY_PARAM		-1001 // Неправильные параметры в функции CreateMemoryForiANPRCapture
#define ERROR_SLOPE_LINE				-1002 // Неверный наклон линии
#define ERROR_NO_PARALLEL_LINES			-1003 // Линии не параллельны
