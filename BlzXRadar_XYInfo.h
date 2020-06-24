#pragma once
//#define _CRT_SECURE_NO_WARNINGS
//#define WINDOWS_IGNORE_PACKING_MISMATCH
//体扫文件头定义
//文件头包括站址段、性能参数段、观测参数段三个部分，共 1266 字节

//文件头：(完整头信息，共1266字节)
#pragma pack(1)
//地址参数段【160字节】---- - 地址参数包括雷达经纬度、天线海拔高度等，共 160 字节
typedef struct AddressParameters
{
	short int FileHeadLength;             	//文件头长度
	char Mode[20];    						//雷达型号
	char Province[20]; 						//省名
	char Area[20]; 							//区名
	char AreaName[20]; 						//区站名
	char VersionNum[20]; 					//文件版本格式号 [4 - 7]存放数据来源
	char TaskName[20]; 						//雷达扫描任务名称
	char NoData1[20]; 						//保留1
	int Longitude; 							//天线经度 单位取1 / 360000度(东经为正，西经为负)
	int Latitude;							//天线纬度 单位取1 / 360000度(北纬为正，南纬为负)
	int Height; 							//天线海拔高度, 以mm为单位
	short int MaxElevate;                   //测站四周地物最大仰角,百分之一度为单位
	short int BestElevate; 					//测站四周地物最佳仰角,百分之一度为单位
	short int NoData2; 						//保留2
}ADDREEPARAMRTRES;

//性能参数段【40字节】---- - 性能参数主要用于雷达本身的性能，共40字节
typedef struct PerformanceParameter
{
	unsigned short  AntennaGain; 			//天线增益，以.01db为单位
	unsigned short  VerticalBW; 			//垂直波束宽度。单位取1/100度
	unsigned short  HorizontalBW; 			//水平波束宽度。单位取1/100度

	unsigned short  Polarizate; 			//极化状况 
											// -0：为水平极化
											// -1：垂直极化
											// -2：为双极化(双偏振)
											// -3：为圆偏振
											// -4：其它

	unsigned int WaveLength; 				//波长，以微米为单位
	unsigned int PeakPower; 				//雷达峰值功率，瓦为单位
	unsigned short  FirstSideLobeLevel;		//第一旁瓣电平，取绝对值(单位取百分之一dB)
	unsigned short  LineA; 					//线性接收机动态范围，百分之一 dB为单位
	unsigned short  AGCDelayNum; 			//AGC延迟量 以微秒为单位
	unsigned short LogA;					//对数接收机，百分之一 dbmw为单位
	unsigned short LineMinTestPower; 		//线性接收机最小可测功率 百分之一，dbwm为单位
	unsigned short NoiseT; 					//噪声消除量化阈值
	unsigned short ClutterT; 				//多普勒杂波消除阈值，单位 .01db 
	unsigned short SQIT; 					//SQI阈值

	unsigned short VelocityP; 				//速度处理方式
											//0：无速度处理
											//1：PPI
											//2：FFT

	unsigned short  FilterP; 				//地物处理方式
											// -0：无地物处理
											// -1：地物杂波图扣除法
											// -2：滤波器处理
											// -3：滤波器+地物杂波图法
											// -4：谱分析法

	unsigned short  IntensityR; 			//强度估算采用的通道
											// -1：对数
											// -2：线性

	unsigned short  iRangeReduction;
}PERFORMANCEPARAMETER;
typedef struct BatchProcessMode
{
	unsigned short int RadarScanneMode;		//雷达扫描模式

	char BSEScanMode[30];					//体扫各层的扫描方式
											// -0：lcs模式(R)
											// -1：lcd模式(V,W)
											// -2：hcd(RVW)
											// -3:
											// -4：batch 模式(RVW)

	unsigned short int  ECDistanceNum[30];	//各层的多普勒距离库数
	char reserve[8];                        //保留位
}BATCHPROCESSODE;
//观测参数段【1066字节】----观测参数记录雷达扫描过程的参数信息，共1066字节
typedef struct observationParameter
{
	unsigned short int ProductNumber; 		//产品编号
											// -0：ppI1
											// -1：RHI
											// -2：立体扫描
											// -3：反射率
											// -4: 速度
											// -5：谱宽

	unsigned short int SSLayerNumber;       //立体扫描层数
	unsigned short int StartYear;			//观测开始时间年
	unsigned short int StartMouth;			//观测开始时间月
	unsigned short int StartDay;			//观测开始时间日
	unsigned short int StartHour;			//观测开始时间时
	unsigned short int StartMinute;			//观测开始时间分
	unsigned short int Startsecond;			//观测开始时间秒
	unsigned int StartGPSTime;				//开始GPS时间，以微秒为单位

	unsigned short int Calibration;			//定标情况
											// -0：没有定标
											// -1：自动定标
											// -2：一周内人工定标
											// -3：一月内人工定标

	unsigned short int IntensityI;			//强度积分次数
	unsigned short int VelocityP;			//速度处理样本数
	unsigned int ID[30];					//ID号

	unsigned char ViewElement[30];			//观测要素
											// -1：单强度
											// -2：单速度(单PRF)
											// -3：速度+谱宽(单PRF)
											// -4：单速度(双PRF)
											// -5：速度+谱宽(双PRF)
											// -6：强度+速度(单PRF)
											// -7：强度+速度(双PRF)
											// -8：三要素(单PRF)
											// -9：三要素(双PRF)
											// -10：四要素(ConR+R+V+W,单PRF)
											// -11：四要素(ConR+R+V+W,双PRF)

	unsigned char SpeedDeambiguity[30];		//速度退模糊
											// -0：无退模糊处理
											// -1：软件退模糊
											// -2：双PRF退模糊
											// -3：批示退模糊
											// -4：批示加软件退模糊
											// -5：双PRF退模糊
											// -6：双PRF+软件退模糊

	unsigned short int EFirstPrr[30];		//各层第一种脉冲重复频率，计数单位1/10HZ
	unsigned short int ESecondPrr[30];		//各层第二种脉冲重复频率，计数单位1/10HZ
	unsigned short int EPulse[30];	 		//各层脉冲宽度，1/100微秒
	unsigned short int EMaxSpeed[30]; 		//各层的最大可测速度，单位：厘米/秒
	unsigned short int ERDistanceNum[30];	//各层的反射率距离库数
	unsigned short int ERadialNum[30];		//各层采样的径向数
	unsigned short int EClutterL[30];		//各层多普勒库长，米为单位
	unsigned short int ERlibL[30];			//各层反射率库长，米为单位
	unsigned short int EStartDistance[30];	//各层径向上的第一个库(或者数据)的开始距离，米为单位
	unsigned int PPIInFileSD[30];			//各层PPI在文件中的开始位置，字节，含文件头
	short int elevation[30];				//各层的仰角，单位/100度
	char RadialArrangement;					//一个径向中的数据排列方式
	unsigned char STOccupysize;				//一个强度数据占用的字节数，百位数表示
	unsigned char SPOccupysize;				//一个速度数据占用的字节数，百位数表示
	unsigned char SWOccupysize;				//一个谱宽数据占用的字节数，百位数表示
	short int STNoEchoCT;					//强度无回波的代码表
	short int SPNoEchoCT;					//速度无回波的代码表
	short int SWNoEchoCT;					//速度无回波的代码表
	short int STMinIncrement;				//数据中的强度最小增量，
	short int SPMinIncrement;				//数据中的速度最小增量，*1000
	short int SWMinIncrement;				//数据中的谱宽最小增量，*1000
	short int Strength;						//强度
	short int speed;						//速度
	short int SpectrumWidth;				//谱宽
	unsigned short int EndYear;				//观测结束时间年
	unsigned short int EndMouth;			//观测结束时间月
	unsigned short int EndDay;				//观测结束时间日
	unsigned short int EndHour;				//观测结束时间时
	unsigned short int EndMinute;			//观测结束时间分
	unsigned short int EndSecond;			//观测结束时间秒
	unsigned int GPSTime;					//GPS时间
	unsigned short int StructNum;			//结构数组的大小

	BATCHPROCESSODE BPM;					//批处理模式
}OBSERVATIONPARAMRTER;
typedef struct VOLFileHeader
{
	ADDREEPARAMRTRES AddressP;				//地址参数段数据结构
	PERFORMANCEPARAMETER PerformanceP;		//性能参数段数据结构
	OBSERVATIONPARAMRTER SurveyP;			//观测参数段数据结构
}VOLFILEHEADER;
typedef struct tag_FLX_LDB
{
	unsigned short spa9B;
	unsigned short spa9A;
	unsigned short spa10B;
	unsigned short spa10A;
	unsigned short spa11B;
	unsigned short spa11A;
	unsigned short spa12B;
	unsigned short spa12A;
	unsigned short spa13B;
	unsigned short spa13A;
	unsigned short ipd;
	unsigned short qpq;
	unsigned short iby;
	unsigned short qby;
}_FLX_LDB;
typedef struct RadioHead
{
	unsigned int space1 : 11;
	unsigned int SIGNAL_PROCESS_OR_STATUS : 5;
	unsigned int RADIAL_NUMBER : 10;
	unsigned int LRF : 1;
	unsigned int space2 : 5;
	unsigned int Az : 16;
	unsigned int el : 16;
	unsigned int PRT_INTERVAL1 : 16;
	unsigned int Current_antenna_rate : 16;
	unsigned int SPOT_BLANK_FLAG : 16;
	unsigned int END_AROUND_TEST;
	unsigned int year : 8;
	unsigned int month : 8;
	unsigned int day : 8;
	unsigned int hour : 8;
	unsigned int minute : 8;
	unsigned int second : 8;
	unsigned int space3 : 16;
	unsigned int space4 : 16;
	unsigned int RECEIVER : 8;
	unsigned int flag : 8;
	unsigned int radioId;
	_FLX_LDB   flx_ldb;
}RADIOHEAD;
#pragma pack(pop)