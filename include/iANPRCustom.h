/******************************************************************************
	(C) 2018. IntBuSoft. http://intbusoft.com
	iANPRCustom.h - version 1.7
*******************************************************************************/
#pragma once

#define FLAG_CUSTOM_MULTI_RESULT		0x01	// Для CUSTOM возвращает несколько наиболее вероятных номеров через "|", первый наиболее вероятный (не больше 3-х)

const int max_countries = 5;
const int max_ianpr_custom_in_types = 20; // масксимальное количество типов номеров для каждой страны

// Это настраиваемый тип распознавания автомобильных номеров
// Каждый элемент характеризует страну. При настройке на C++
// можно выделять и удалять объеты настраиваемого типа самостоятельно
// Для других языков рекомендуется для формирования использовать XML
// интерфейс. А для удаления 0 функцию из данного файла
struct iANPRCustomElement
{
	int all_types;
	float probability;                                  // Вероятность страны
	int types[max_ianpr_custom_in_types];               // Типы берутся в iANPR
	float probability_types[max_ianpr_custom_in_types]; // Вероятность каждого типа, можно не заполнять
};
struct iANPRCustom
{
	int all_countries;
	iANPRCustomElement* Elements;
	int flags; // По умолчанию 0 
};

// Функция создает указатель на iANPRCustom из буфера xml
#ifndef WIN32
__attribute__((visibility("default")))
#endif
void* 
#ifdef WIN32
__stdcall
#endif
CreateiANPRCustom(char* xml_buffer, int buffer_size);

// Функция удаляет буфер
void 
#ifdef WIN32
__stdcall
#else
__attribute__((visibility("default")))
#endif
DeleteiANPRCustom(void* xml);
