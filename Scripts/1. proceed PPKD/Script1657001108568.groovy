import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil

request_search = WS.sendRequest(findTestObject('AID-Transaksi/1. PPKD - Search Realisasi', [('moid') : '20000012']))

//store json response to variable
def slurper_search = new groovy.json.JsonSlurper()

def parsedJson_count = slurper_search.parseText(request_search.getResponseText())

// count array size
def expectedSize = parsedJson_count.size()

def search = slurper_search.parseText(request_search.getResponseBodyContent())

// get var in console
KeywordUtil.logInfo('jumlah is :' + expectedSize)

// get spesific array
def spesific = search.SearchRealisasi.findAll({ Map map ->
        map.get('DealerName').contains('Asia Berjaya Mobilindo')
    })

println(spesific)

//  ^^^^^^^^^^^^^^^^^
def accno = spesific[0].NoKontrak

println(accno)

// store norek
String norek = accno.substring(0, 10)

// store nopin
String nopin = accno.substring(11, 14)

//get realisasi
request_real = WS.sendRequest(findTestObject('AID-Transaksi/2. PPKD - Get Realisasi', [('norek') : norek, ('nopin') : nopin]))

def slurper_real = new groovy.json.JsonSlurper()

def get_real = slurper_real.parseText(request_real.getResponseBodyContent())

// store year
def thn_kendaraan = get_real[0].Year

// store dealer id
def dealerid = get_real[0].DealerId

// store packet id
def packetid = get_real[0].PacketId

// store car condition
def car_condition = get_real[0].CarCondition

// store tgl realisasi
def tgl_real_date_time = get_real[0].tglrealisasi

def tgl_real_date = tgl_real_date_time.substring(0, tgl_real_date_time.length() - 9 // Removes last character
    )

def tgl_real = tgl_real_date.replaceAll('[- ]', '' // Removes '-'
    )

println(tgl_real)

// store tgl printing
def tgl_printing_date_time = get_real[0].tglproceedprinting

def tgl_printing_date = tgl_printing_date_time.substring(0, tgl_printing_date_time.length() - 13 // Removes last character
    )

def tgl_printing = tgl_printing_date.replaceAll('[- ]', '' // Removes '-'
    )

println(tgl_printing)

// validasi entry data
request_id = WS.sendRequest(findTestObject('AID-Transaksi/3. PPKD - Validasi Entry Data', [(' norek') : norek, (' nopin') : nopin
            , ('year') : thn_kendaraan]))

def slurper_id = new groovy.json.JsonSlurper()

def val_entry = slurper_id.parseText(request_id.getResponseBodyContent())

KeywordUtil.logInfo("$val_entry")

//get id penerima
request_idpenerima = WS.sendRequest(findTestObject('AID-Flow 1/List Daftar Penerima', [('dealerid') : dealerid]))

def slurper_idpenerima = new groovy.json.JsonSlurper()

def search_penerima = slurper_idpenerima.parseText(request_idpenerima.getResponseBodyContent())

// get spesific array
def spesific_penerima = search_penerima.DaftarPenerima.findAll({ Map map ->
        map.get('NamaPenerima').contains(nama_penerima)
    })

// store id penerima
def id_penerima = spesific_penerima[0].IdPenerima

println(id_penerima)

//get norekening penerima
request_rekening = WS.sendRequest(findTestObject('AID-Flow 1/Detail Daftar Penerima', [('id_penerima') : id_penerima]))

//store json response to variable
def slurper_rekening = new groovy.json.JsonSlurper()

def result_rekening = slurper_rekening.parseText(request_rekening.getResponseText())

def no_rekening = result_rekening.Account[0].AccountNo

println(no_rekening)

//get nominal komisi asuransi (tanya EKO )
request_komisi_ins = WS.sendRequest(findTestObject('AID-Transaksi/4. PPKD - Asuransi Dibagikan', [('accno') : accno, ('id') : id_penerima
            , ('no_rekening') : no_rekening, ('percent') : percent_ins_bag, ('komisi') : 0]))

def slurper_komisi_ins_bag = new groovy.json.JsonSlurper()

def get_ins_bag = slurper_komisi_ins_bag.parseText(request_komisi_ins.getResponseBodyContent())

def amount_ins_bag_trim = get_ins_bag.NominalKomisi

def amount_ins_bag = amount_ins_bag_trim.replaceAll('[, ]', '' // Removes ','
    )

println(amount_ins_bag)

//get nominal komisi asuransi dicadangkan (tanya EKO )
request_komisi_cad = WS.sendRequest(findTestObject('AID-Transaksi/5. PPKD - Asuransi Dicadangkan', [('jenis_cad') : jenis_asuransi_cad
            , ('accno') : accno, ('percent_ins_cad') : percent_ins_cad, ('komisi_amnt') : 0]))

def slurper_komisi_ins_cad = new groovy.json.JsonSlurper()

def get_ins_cad = slurper_komisi_ins_cad.parseText(request_komisi_cad.getResponseBodyContent())

def amount_ins_cad_trim = get_ins_cad.NominalKomisi

def amount_ins_cad = amount_ins_cad_trim.replaceAll('[, ]', '' // Removes ','
    )

println(amount_ins_cad)

//suku bunga dealer - get refund rate
request_suku_bunga = WS.sendRequest(findTestObject('AID-Transaksi/6. PPKD - Sukubunga Dealer', [('accno') : accno, ('year') : thn_kendaraan
            , ('suku_bunga_dealer') : suku_bunga_dealer]))

def slurper_suku_bunga = new groovy.json.JsonSlurper()

def get_suku_bunga = slurper_suku_bunga.parseText(request_suku_bunga.getResponseBodyContent())

def amount_refund_rate = get_suku_bunga.RefundRateNominal

def percent_refund_rate = get_suku_bunga.RefundRatePersen

//println(percent_refund_rate)

//println(amount_refund_rate)

//refund rate dibagikan
request_refund_bag = WS.sendRequest(findTestObject('AID-Transaksi/7. PPKD - Refund Rate Dibagikan', [('accno') : accno, ('id_penerima') : id_penerima
            , ('norekening') : no_rekening, ('refundrate_percent') : percent_refund_rate, ('sukubunga') : 11, ('amount_komisi') : '0']))

def slurper_refund_bag = new groovy.json.JsonSlurper()

def get_refund_bag = slurper_refund_bag.parseText(request_refund_bag.getResponseBodyContent())

def amount_refund_bag = get_refund_bag.NominalKomisi

//println(amount_refund_bag)
//refund rate dicadangkan
request_refund_cad = WS.sendRequest(findTestObject('AID-Transaksi/8. PPKD - Refund Rate Dicadangkan', [('accno') : accno
            , ('jenis_refundrate_cad') : jenis_refundrate_cad, ('refundrate_percent') : percent_refund_rate, ('sukubunga') : 11
            , ('amnt_komisi') : 0]))

def slurper_refund_cad = new groovy.json.JsonSlurper()

def get_refund_cad = slurper_refund_cad.parseText(request_refund_cad.getResponseBodyContent())

def amount_refund_cad = get_refund_cad.NominalKomisi

//println(amount_refund_cad)
//request inatura provisi dibagikan -- belum bisa
request_inatura_provisi_bag = WS.sendRequest(findTestObject('AID-Transaksi/9. PPKD - Inatura Provisi Dibagikan', [('accno') : accno
            , ('id_penerima') : id_penerima, ('no_rekening') : no_rekening, ('persentase') : percent_inatura_provisi_bag
            , ('amnt_komisi') : 0]))

def slurper_inatura_provisi_bag = new groovy.json.JsonSlurper()

def get_inatura_provisi_bag = slurper_inatura_provisi_bag.parseText(request_inatura_provisi_bag.getResponseBodyContent())

//def inatura_provisi_bag = get_inatura_provisi_bag.NominalKomisi
//println(get_inatura_provisi_bag)
//request inatura provisi dicadangkan
request_inatura_provisi_cad = WS.sendRequest(findTestObject('AID-Transaksi/910. PPKD - Inatura Provisi Dicadangkan', [('nokontrak') : accno
            , ('jenis_cad') : jenis_asuransi_cad, ('persentase') : percent_inatura_provisi_cad, ('amount_komisi') : 0]))

def slurper_inatura_provisi_cad = new groovy.json.JsonSlurper()

def get_inatura_provisi_cad = slurper_inatura_provisi_cad.parseText(request_inatura_provisi_cad.getResponseBodyContent())

def amount_inatura_provisi_cad = get_inatura_provisi_cad.NominalKomisi

//println(amount_inatura_provisi_cad)
//request master inatura program : product id ( CS New = N, CS Used = U, Direct Sales = D, KKB = K )
request_master_inatura_program = WS.sendRequest(findTestObject('AID-Transaksi/911. Master Inatura Program', [('id') : packetid
            , ('productid') : 'N', ('car_condition') : car_condition, ('tgl_printing') : tgl_printing, ('tgl_real') : tgl_real]))

def slurper_master_inatura_program = new groovy.json.JsonSlurper()

def get_master_inatura_program = slurper_master_inatura_program.parseText(request_master_inatura_program.getResponseBodyContent())

//println(get_master_inatura_program)
def id_inatura_program = get_master_inatura_program[0].InaturaProgramID

def amount_inatura_program = get_master_inatura_program[0].InaturaProgramAmount

println(amount_inatura_program)

//request inatura program dibagikan
request_inatura_program_bag = WS.sendRequest(findTestObject('AID-Transaksi/912. PPKD - Inatura Program Dibagikan', [('accno') : accno
            , ('idpenerima') : id_penerima, ('norek') : no_rekening, ('inatura_program') : id_inatura_program, ('amnt_komisi') : amount_inatura_program]))

def slurper_inatura_program_bag = new groovy.json.JsonSlurper()

def get_inatura_program_bag = slurper_inatura_program_bag.parseText(request_inatura_program_bag.getResponseBodyContent())

def amount_inatura_program_bag = get_inatura_program_bag.NominalKomisi

println(amount_inatura_program_bag)

//request inatura program dicadangkan
request_inatura_program_cad = WS.sendRequest(findTestObject('AID-Transaksi/913. PPKD - Inatura Program Dicadangkan', [('nokontrak') : accno
            , ('jenis_cad') : jenis_inatura_program_cad, ('persentase') : percent_inatura_program_cad, ('amount_komisi') : 0
            , ('id_inatura_program') : id_inatura_program]))

def slurper_inatura_program_cad = new groovy.json.JsonSlurper()

def get_inatura_program_cad = slurper_inatura_program_cad.parseText(request_inatura_program_cad.getResponseBodyContent())

def amount_inatura_program_cad = get_inatura_program_cad.NominalKomisi

println(amount_inatura_program_cad)

//request komisi admin dibagikan
request_komisi_admin_bag = WS.sendRequest(findTestObject('AID-Transaksi/914. PPKD - Komisi Admin Dibagikan', [('nokontrak') : accno
            , ('id_penerima') : id_penerima, ('no_rekening') : no_rekening, ('persentase') : percent_komisi_admin_bag, ('amount') : 0]))

def slurper_komisi_admin_bag = new groovy.json.JsonSlurper()

def get_komisi_admin_bag = slurper_komisi_admin_bag.parseText(request_komisi_admin_bag.getResponseBodyContent())

def amount_komisi_admin_bag = get_komisi_admin_bag.NominalKomisi

println(amount_komisi_admin_bag)

//request komisi admin dicadangkan
request_komisi_admin_cad = WS.sendRequest(findTestObject('AID-Transaksi/915. PPKD - Komisi Admin Dicadangkan', [('accno') : accno
            , ('jenis_cad') : jenis_komisi_admin_cad, ('persentase') : percent_komisi_admin_cad, ('amount') : 0]))

def slurper_komisi_admin_cad = new groovy.json.JsonSlurper()

def get_komisi_admin_cad = slurper_komisi_admin_cad.parseText(request_komisi_admin_cad.getResponseBodyContent())

def amount_komisi_admin_cad = get_komisi_admin_cad.NominalKomisi

println(amount_komisi_admin_bag)

//request rule maks asuransi
request_rule_max_ins = WS.sendRequest(findTestObject('AID-Transaksi/916. PPKD - Rule Maksimum Asuransi', [('accno') : accno
            , ('year') : thn_kendaraan, ('amount_komisi_cad') : amount_ins_bag, ('amount_komisi_bag') : amount_ins_cad]))

def slurper_rule_max_ins = new groovy.json.JsonSlurper()

def get_rule_max_ins = slurper_rule_max_ins.parseText(request_rule_max_ins.getResponseBodyContent())

def rule_max_ins_message = get_rule_max_ins.Message[6]

println(rule_max_ins_message)

//verify rule max ins
WS.verifyElementPropertyValue(request_rule_max_ins, 'Message[6]', 'Grand Total Insentif Asuransi anda tidak melewati rule maksimum', 
    FailureHandling.CONTINUE_ON_FAILURE)

//request rule maks refundrate
request_rule_max_refundrate = WS.sendRequest(findTestObject('AID-Transaksi/917. PPKD - Rule Maksimum RefundRate', [('accno') : accno
            , ('year') : thn_kendaraan, ('komisi_bag') : amount_refund_bag, ('komisi_cad') : amount_refund_rate, ('suku_bunga_dealer') : suku_bunga_dealer
            , ('percent_refund_rate') : 0.0, ('amnt_refund_rate') : 0]))

def slurper_rule_max_refundrate = new groovy.json.JsonSlurper()

def get_rule_max_refundrate = slurper_rule_max_refundrate.parseText(request_rule_max_refundrate.getResponseBodyContent())

def rule_max_refundrate_message = get_rule_max_ins.Message[7]

println(rule_max_refundrate_message)

WS.verifyElementPropertyValue(request_rule_max_refundrate, 'Message[7]', 'Grand Total Refund Rate anda tidak melewati rule maksimum', 
    FailureHandling.CONTINUE_ON_FAILURE)

//request rule maks inatura provisi = inatura belum bisa jadi hardcode
request_rule_max_inatura_prov = WS.sendRequest(findTestObject('AID-Transaksi/918. PPKD - Rule Maksimum InaturaProvisi', 
        [('accno') : accno, ('year') : thn_kendaraan, ('komisi_bag') : 0, ('komisi_cad') : amount_inatura_provisi_cad]))

def slurper_rule_max_inatura_prov = new groovy.json.JsonSlurper()

def get_rule_max_inatura_prov = slurper_rule_max_inatura_prov.parseText(request_rule_max_inatura_prov.getResponseBodyContent())

def rule_max_inatura_prov_message = get_rule_max_inatura_program.Message[8]

println(rule_max_inatura_prov_message)

WS.verifyElementPropertyValue(request_rule_max_refundrate, 'Message[8]', 'Grand Total Inatura Provisi anda tidak melewati rule maksimum', 
    FailureHandling.CONTINUE_ON_FAILURE)

//request rule max inatura program
request_rule_max_inatura_program = WS.sendRequest(findTestObject('AID-Transaksi/919. PPKD - Rule Maksimum InaturaProgram', 
        [('accno') : accno, ('year') : thn_kendaraan, ('id_inatura') : id_inatura_program, ('komisi_bag') : amount_inatura_program_bag
            , ('komisi_cad') : amount_inatura_program_cad]))

def slurper_rule_max_inatura_program = new groovy.json.JsonSlurper()

def get_rule_max_inatura_program = slurper_rule_max_inatura_program.parseText(request_rule_max_inatura_program.getResponseBodyContent())

def rule_max_inatura_program_message = get_rule_max_inatura_program.Message[6]

println(rule_max_inatura_program_message)

WS.verifyElementPropertyValue(request_rule_max_refundrate, 'Message[6]', 'Grand Total Inatura Program anda tidak melewati rule maksimum', 
    FailureHandling.CONTINUE_ON_FAILURE)

//request rula maks insentif admin
request_rule_max_insentif_adm = WS.sendRequest(findTestObject('AID-Transaksi/920. PPKD - Rule Maksimum InsentifAdmin', [('accno') : accno, ('year') : thn_kendaraan
            , ('komisi_bag') : amount_komisi_admin_bag, ('komisi_cad') : amount_komisi_admin_cad]))

def slurper_rule_max_insentif_adm = new groovy.json.JsonSlurper()

def get_rule_max_insentif_adm = slurper_rule_max_insentif_adm.parseText(request_rule_max_insentif_adm.getResponseBodyContent())

def rule_max_insentif_adm_message = get_rule_max_insentif_adm.Message[6]

println(rule_max_insentif_adm_message)

WS.verifyElementPropertyValue(request_rule_max_insentif_adm, 'Message[6]', 'Grand Total Insentif Admin anda tidak melewati rule maksimum',
	FailureHandling.CONTINUE_ON_FAILURE)


