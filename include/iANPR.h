/******************************************************************************
	(C) 2018. IntBuSoft. http://intbusoft.com
	iANPR.h - version 1.7
*******************************************************************************/
#pragma once

#include "iANPRerror.h"
#include "iANPRCustom.h"
#include "opencv2/core/core_c.h"
#include "opencv2/imgproc/imgproc_c.h"


// Методы детектирования автомобильного номера
// Методы, основанные на детектировании номера целиком
#define ANPR_DETECTMODE1			0x01  // Простая адаптивная обработка
#define ANPR_DETECTMODE2			0x02  // Адаптивная обработка с дополнительной фильтрацией изображения
#define ANPR_DETECTMODE3			0x04  // Блочная адаптивная обработка
// Методы, основанные на выделении частей номера
#define ANPR_DETECTMODE4			0x08  // Простая адаптивная обработка с внутренним анализом	номера

#define ANPR_DETECTCOMPLEXMODE            (ANPR_DETECTMODE2 | ANPR_DETECTMODE3 | ANPR_DETECTMODE4)

// Типы распознаваемых автомобильных номеров

// Цветное изображение
#define ANPR_CUSTOM_TYPE				-1 // Настраиваемый тип автомобильного номера

// На вход поступает изображение в градациях серого
#define	ANPR_RUSSIAN_BASE				0  // Базовые
#define	ANPR_RUSSIAN_BASE2				1  // Базовые и транзитные
#define	ANPR_RUSSIAN_EXTENDED			2  // Базовые, транзитные и номера прицепов
#define ANPR_RUSSIAN_PUBLIC				3  // Только общественный транспорт
#define ANPR_RUSSIAN_POLICE				5  // Только номера полиции
#define ANPR_RUSSIAN_ARMY				6  // Только военные номера
#define ANPR_RUSSIAN_SQUARE_BASE		8  // Российские двухстрочные номера мотоциклов и тракторов
#define ANPR_RUSSIAN_DIPLOMAT			10  // Российские дипломатические номера
// ВНИМАНИЕ!!! Следующие конфигурации работают только для цветных изображений
#define	ANPR_RUSSIAN_EXTENDED2			4  // Базовые, транзитные и номера прицепов и ANPR_RUSSIAN_PUBLIC
#define	ANPR_RUSSIAN_FULL				7  // ANPR_RUSSIAN_EXTENDED2 + ANPR_RUSSIAN_POLICE + ANPR_RUSSIAN_ARMY + ANPR_RUSSIAN_DIPLOMAT
#define	ANPR_RUSSIAN_FULL_WITH_SQUARE	9  // ANPR_RUSSIAN_FULL + ANPR_RUSSIAN_SQUARE_BASE

// Номера Казахстана
// На вход поступает изображение в градациях серого
#define ANPR_KAZ_1993_PRIVATE			100
#define ANPR_KAZ_1993_ORGANIZATION		101
#define ANPR_KAZ_2012_PRIVATE			102
#define ANPR_KAZ_2012_ORGANIZATION		103
#define ANPR_KAZ_BASE					104   // ANPR_KAZ_1993_PRIVATE + ANPR_KAZ_1993_ORGANIZATION + ANPR_KAZ_2012_PRIVATE + ANPR_KAZ_2012_ORGANIZATION

// Номера Туркменистана
// На вход поступает изображение в градациях серого
#define ANPR_TM_2009					201
#define ANPR_TM_PRIVATE_BEFORE_2009		202
#define ANPR_TM_BASE					203  // ANPR_TM_2009 + ANPR_TM_PRIVATE_BEFORE_2009

// Номера Беларуси
#define ANPR_BY_TRUCK					300 // ANPR_BY_2004_TRUCK + ANPR_BY_1992_TRUCK
#define ANPR_BY_2004_TRUCK				301 // Грузовые автомобили и автобусы с 2004г.
#define ANPR_BY_1992_TRUCK				302 // Грузовые автомобили и автобусы с 1992г.
#define ANPR_BY_2004_TRAILER			303 // Задний знак для прицепов и полуприцепов с 2004 г.
#define ANPR_BY_2004_BASE				304 // Легковые автомобили, начиная с 2004 г.
#define ANPR_BY_SQUARE_BASE				305 // Задние двухстрочные знаки, начиная с 2004г.
#define ANPR_BY_2004_BASE2				306 // ANPR_BY_2004_TRUCK + ANPR_BY_2004_BASE + ANPR_BY_2004_TRAILER
#define ANPR_BY_TRANSIT					307 // Транзитные номера
#define ANPR_BY_PUBLIC					308 // Общественный транспорт
#define ANPR_BY_POLICE					309 // Номера МВД
#define ANPR_BY_FULL					310 // ANPR_BY_2004_BASE2 + ANPR_BY_TRANSIT + ANPR_BY_PUBLIC + ANPR_BY_POLICE
#define ANPR_BY_FULL_WITH_SQUARE		311 // ANPR_BY_FULL + ANPR_BY_SQUARE_BASE
#define ANPR_BY_2004_BASE3				312 // ANPR_BY_2004_BASE2 + ANPR_BY_TRANSIT

// Номера Польши
#define ANPR_PL_BASE					400 // Стандартные номера Польши ANPR_PL_BASE_7 | ANPR_PL_BASE_8
#define ANPR_PL_BASE_7					401 // Стандартные номера Польши 7 символов
#define ANPR_PL_BASE_8					402 // Стандартные номера Польши 8 символов

// Номера Латвии
#define ANPR_LV_BASE					500 // Стандартные номера 1992 и 2004 года

// Номера Литвы
#define ANPR_LT_BASE					600 // Стандартные номера с 2004 года

// Номера Эстонии
#define ANPR_EST_BASE					700 // Стандартные номера 2004 года

// Номера Украины
#define ANPR_UA_BASE					800 // Стандартные номера 2015 года (автомобили, прицепы, автобусы)
#define ANPR_UA_TRANSIT					801 // Номера для разовых поездок на автомобилях, прицепах к ним и автобусах, 2015 год
#define ANPR_UA_DIPLOMAT				802 // Знаки для автотранспорта сотрудников дипломатических представительств, консульств, представительств международных организаций, обладающих дипломатическим статусом + номера техперсонала. С 1 июля 2013 года
#define ANPR_UA_BASE2					803 // ANPR_UA_BASE + ANPR_UA_DIPLOMAT
#define ANPR_UA_BASE3					804 // ANPR_UA_BASE2 + ANPR_UA_TRANSIT

// Номера Молдовы
#define ANPR_MD_2011_BASE				900 // Номера физических и юридических лиц 2011-2015 года + номера такси
#define ANPR_MD_2011_TRAILER			901 // Номера прицепов и полуприцепов 2011-2015 года
#define ANPR_MD_2011_SQUARE_BASE		902 // Номера мототехники 2011-2015 года
#define ANPR_MD_2011_BASE2				903 // ANPR_MD_2011_BASE + ANPR_MD_2011_TRAILER
#define ANPR_MD_2011_FULL_WITH_SQUARE	911 // ANPR_MD_2011_BASE2 + ANPR_MD_2011_SQUARE_BASE

// Список флагов, при которых необходимо передавать на распознавание не изображение в оттенках серого (одноканальное), а цветное (многоканальное) изображение.
// Во всех остальных случаях необходимо изображение в оттенках серого (одноканальное)
static const int anprFullTypesCount = 6;
static const int anprFullTypes [anprFullTypesCount] = 
{
	ANPR_RUSSIAN_EXTENDED2, ANPR_RUSSIAN_FULL, ANPR_RUSSIAN_FULL_WITH_SQUARE,
	ANPR_BY_FULL, ANPR_BY_FULL_WITH_SQUARE,
	ANPR_MD_2011_FULL_WITH_SQUARE
};

// Заполнение flags в ANPR_OPTIONS
#define DEBUG_RECOGNITION_MODE			0x01  // Будут выводиться все распознанные номера, в том числе и с неопределенными символами (знак вопроса)
#define RETURN_TYPE_NUMBER				0x02  // Возвратить тип номера, который в этом случае будет записываться через двоеточие в конце номера
											  // Буфер должен в этом случае принимать на 4 символа больше, чем максимальный размер номера
#define NO_LOW_RELIABILITY				0x08  // Не возвращает номера с низкой достоверностью
#define ONLY_HORIZONTAL_NUMBER			0x04  // Только расположенные правильно перед камерой номера, наклонные номера игнорируются
                                              // не работает для ANPR_DETECTMODE4
#define IR_LIGHTING_CAMERA				0x10  // Фон белый, а цифры чёрные

// Возвращаемые типы номеров (если тип не возвращается, то значит он или не определен для данной страны, или в буфере для его записи не хватает места)
// -1 возвращается при неопределенном типе
#define TYPE_RUSSIAN_BASE				0	// Базовый номер России
#define TYPE_RUSSIAN_TRANSIT			1	// Транзитный номер России
#define TYPE_RUSSIAN_TRAILER			2	// Номер прицепа России
#define TYPE_RUSSIAN_PUBLIC				3	// Общественный транспорт России
#define TYPE_RUSSIAN_POLICE				4	// Номер полиции России
#define TYPE_RUSSIAN_ARMY				5	// Военный номер России
#define TYPE_RUSSIAN_SQUARE_BASE		6	// Номер трактора или мотоцикла России
#define TYPE_RUSSIAN_DIPLOMAT			7  // Российские дипломатические номера
#define TYPE_TM_BASE					10
#define TYPE_KAZ_PRIVATE1993			21  
#define TYPE_KAZ_ORGANIZATION1993		22  
#define TYPE_KAZ_PRIVATE2012			23  
#define TYPE_KAZ_ORGANIZATION2012		24  
#define TYPE_KAZ_BASE					25
#define TYPE_BY_2004_BASE				30	// Базовый номер Республики Беларусь
#define TYPE_BY_TRANSIT					31	// Транзитный номер Республики Беларусь
#define TYPE_BY_2004_TRAILER			32	// Номер прицепа Республики Беларусь
#define TYPE_BY_PUBLIC					33	// Общественный транспорт Республики Беларусь
#define TYPE_BY_POLICE					34	// Номер полиции Республики Беларусь
#define TYPE_BY_ARMY					35	// Военный номер Республики Беларусь
#define TYPE_BY_SQUARE_BASE				36	// Квадратные номера Республики Беларусь
#define TYPE_BY_2004_TRUCK				37	// Грузовые автомобили и автобусы Республики Беларусь c 2004
#define TYPE_BY_1992_TRUCK				38	// Грузовые автомобили и автобусы Республики Беларусь c 1992
#define TYPE_PL_BASE					40	// Стандартные номера Польши (3 буквы, 2 цифры, 2 буквы), 2000 и 2006 год
#define TYPE_LV_BASE					50	// Стандартные номера Латвии 1992 и 2004 года
#define TYPE_LT_BASE					60	// Стандартные номера Литвы с 2004 года
#define TYPE_EST_BASE					70	// Стандартные номера Эстонии 2004 года
#define TYPE_UA_BASE					80	// Стандартные номера Украины 2015 года (легковые автомобили, прицепы, автобусы)
#define TYPE_UA_TRANSIT					81	// Номера для разовых поездок на автомобилях, прицепах к ним и автобусах, 2015 год
#define TYPE_UA_DIPLOMAT				82	// Знаки для автотранспорта сотрудников дипломатических представительств, консульств, представительств международных организаций, обладающих дипломатическим статусом + номера техперсонала. С 1 июля 2013 года
#define TYPE_MD_2011_BASE				90	// Номера физических и юридических лиц 2011-2015 года
#define TYPE_MD_2011_TRAILER			91	// Номера прицепов и полуприцепов 2011-2015 года
#define TYPE_MD_2011_SQUARE_BASE		92	// Номера мототехники 2011-2015 года

struct ANPR_OPTIONS
{
	char sign1 = 'i', sign2 = 'a', sign3 = '1';	 // служебная информация, не должна модифицироваться

	int min_plate_size;  // Минимальная площадь номера
	int max_plate_size;  // Максимальная площадь номера
	int Detect_Mode;     // Режимы детектирования	
	int max_text_size;   // Максимальное количество символов номера + 1
	int type_number;     // Тип автомобильного номера 
	int flags;           // Дополнительные опции
	void* custom;		// Заполняется только для типа ANPR_CUSTOM_TYPE, иначе пусто
	/*
		void, а не iANPRCustom по причине того, чтобы в не С/C++ языках не нужно было объявлять структуру данных
	*/
	char* vers = "1.7"; // Используемая версия iANPR SDK. Если не задано (vers = 0), то считается 1.5.

	// Для определения углов поворота вдоль осей воспользуйтесь утилитой persptrans, которую можно найти в составе iANPR SDK.
	// 90.0 означает отсутствие поворота
	double alpha = 90.0;			// Поворот вокруг оси X
	double beta = 90.0;			// Поворот вокруг оси Y
	double gamma = 90.0;			// Поворот вокруг оси Z

	int max_threads = 1;		// Количество потоков
};


// Типы для CUSTOM
// Россия
#define CUSTOM_RUSSIAN_BASE_EXTENDED		ANPR_RUSSIAN_EXTENDED
#define CUSTOM_RUSSIAN_PUBLIC				ANPR_RUSSIAN_PUBLIC	
#define CUSTOM_RUSSIAN_POLICE				ANPR_RUSSIAN_POLICE
#define CUSTOM_RUSSIAN_ARMY					ANPR_RUSSIAN_ARMY
#define CUSTOM_RUSSIAN_SQUARE				ANPR_RUSSIAN_SQUARE_BASE
#define CUSTOM_RUSSIAN_DIPLOMAT				ANPR_RUSSIAN_DIPLOMAT
// Казахстан
#define CUSTOM_KAZ_1993_PRIVATE				ANPR_KAZ_1993_PRIVATE
#define CUSTOM_KAZ_1993_ORGANIZATION		ANPR_KAZ_1993_ORGANIZATION
#define CUSTOM_KAZ_2012_PRIVATE				ANPR_KAZ_2012_PRIVATE
#define CUSTOM_KAZ_2012_ORGANIZATION		ANPR_KAZ_2012_ORGANIZATION
// Беларусь
#define CUSTOM_BY_2004_BASE2				ANPR_BY_2004_BASE2
#define CUSTOM_BY_TRANSIT					ANPR_BY_TRANSIT
#define CUSTOM_BY_PUBLIC					ANPR_BY_PUBLIC
#define CUSTOM_BY_POLICE					ANPR_BY_POLICE
#define CUSTOM_BY_SQUARE_BASE				ANPR_BY_SQUARE_BASE
// Польша
#define CUSTOM_PL_BASE_7					ANPR_PL_BASE_7
#define CUSTOM_PL_BASE_8					ANPR_PL_BASE_8
// Латвия
#define CUSTOM_LV_BASE						ANPR_LV_BASE
// Литва
#define CUSTOM_LT_BASE						ANPR_LT_BASE
// Эстония
#define CUSTOM_EST_BASE						ANPR_EST_BASE
// Молдова
#define CUSTOM_MD_2011_BASE					ANPR_MD_2011_BASE
#define CUSTOM_MD_2011_TRAILER				ANPR_MD_2011_TRAILER
#define CUSTOM_MD_2011_SQUARE_BASE			ANPR_MD_2011_SQUARE_BASE
// Украина
#define CUSTOM_UA_BASE						ANPR_UA_BASE
#define CUSTOM_UA_TRANSIT					ANPR_UA_TRANSIT
#define CUSTOM_UA_DIPLOMAT					ANPR_UA_DIPLOMAT

int
#ifdef WIN32
__stdcall 
#else
__attribute__ ((visibility("default")))
#endif 
anprPlate( IplImage* Image, ANPR_OPTIONS Options, int* AllNumber, CvRect* Rects, char** Texts, void* param = NULL );

int
#ifdef WIN32
__stdcall 
#else
__attribute__ ((visibility("default")))
#endif 
anprPlateRect( IplImage* Image, CvRect Rect, ANPR_OPTIONS Options, int* AllNumber, CvRect* Rects, char** Texts, void* param = NULL );

void 
#ifdef WIN32
__stdcall 
#else
__attribute__ ((visibility("default")))
#endif 
LicenseValue( char* lic );

int
#ifdef WIN32
__stdcall
#else
__attribute__((visibility("default")))
#endif 
createIANPROptions(char charArray[sizeof (ANPR_OPTIONS)], char sing1, char sing2, char sing3, int min_plate_size, int max_plate_size, int Detect_Mode,
int max_text_size, int type_number, int flags, void* custom, char* vers, double alpha, double beta, double gamma, int max_threads);