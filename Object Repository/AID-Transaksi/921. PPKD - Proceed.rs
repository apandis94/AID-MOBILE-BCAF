<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>921. PPKD - Proceed</name>
   <tag></tag>
   <elementGuidId>68749bca-b817-4018-81a0-a0b940b887d3</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;NoRek\&quot;: \&quot;9530708518\&quot;,\n  \&quot;NoPIN\&quot;: \&quot;001\&quot;,\n  \&quot;BranchID\&quot;: \&quot;9530\&quot;,\n  \&quot;Year\&quot;: \&quot;2022\&quot;,\n  \&quot;tglRealisasi\&quot;: \&quot;2022-06-21T00:00:00\&quot;,\n  \&quot;UserID\&quot;: \&quot;20000012\&quot;,\n  \&quot;PermohonanPembayaranKomisiDealerHeader\&quot;: {\n    \&quot;SukuBungaDealer\&quot;: 1.0,\n    \&quot;InaturaProgramID\&quot;: 2,\n    \&quot;RuleMaksimumPersenInsentifAsuransi\&quot;: 3.0,\n    \&quot;RuleMaksimumAmountInsentifAsuransi\&quot;: 4.0,\n    \&quot;RuleMaksimumPersenRefundRate\&quot;: 5.0,\n    \&quot;RuleMaksimumAmountRefundRate\&quot;: 6.0,\n    \&quot;RuleMaksimumPersenInaturaProvisi\&quot;: 7.0,\n    \&quot;RuleMaksimumAmountInaturaProvisi\&quot;: 8.0,\n    \&quot;RuleMaksimumAmountInsentifAdmin\&quot;: 9.0,\n    \&quot;CadanganCMCDocNoInsentifAsuransi\&quot;: \&quot;sample string 10\&quot;,\n    \&quot;CadanganCMCDocNoRefundRate\&quot;: \&quot;sample string 11\&quot;,\n    \&quot;CadanganCMCDocNoInaturaProvisi\&quot;: \&quot;sample string 12\&quot;,\n    \&quot;CadanganCMCDocNoInaturaProgram\&quot;: \&quot;sample string 13\&quot;,\n    \&quot;CadanganCMCDocNoInsentifAdmin\&quot;: \&quot;sample string 14\&quot;\n  },\n  \&quot;CadanganCMC\&quot;: {\n    \&quot;PersenCadanganCMCInsentifAsuransi\&quot;: 1.0,\n    \&quot;NominalCadanganCMCInsentifAsuransi\&quot;: 2.0,\n    \&quot;PersenCadanganCMCRefundRate\&quot;: 3.0,\n    \&quot;NominalCadanganCMCRefundRate\&quot;: 4.0,\n    \&quot;PersenCadanganCMCInaturaProvisi\&quot;: 5.0,\n    \&quot;NominalCadanganCMCInaturaProvisi\&quot;: 6.0,\n    \&quot;PersenCadanganCMCInaturaProgram\&quot;: 7.0,\n    \&quot;NominalCadanganCMCInaturaProgram\&quot;: 8.0,\n    \&quot;PersenCadanganCMCInsentifAdmin\&quot;: 9.0,\n    \&quot;NominalCadanganCMCInsentifAdmin\&quot;: 10.0\n  },\n  \&quot;KomisiAsuransiyangDibagikan\&quot;: [\n    {\n      \&quot;IdPenerima\&quot;: 1003,\n      \&quot;AccountNo\&quot;: \&quot;1340420175\&quot;,\n      \&quot;PersentaseInsentifDibagikan\&quot;: 25.0000,\n      \&quot;NominalInsentifDibagikan\&quot;: 4306750.0000\n    }\n  ],\n  \&quot;KomisiAsuransiyangDicadangkan\&quot;: [\n    {\n      \&quot;JenisCadangan\&quot;: \&quot;sample string 1\&quot;,\n      \&quot;PersentaseInsentifDicadangkan\&quot;: 2.0,\n      \&quot;NominalInsentifDicadangkan\&quot;: 3.0\n    },\n    {\n      \&quot;JenisCadangan\&quot;: \&quot;sample string 1\&quot;,\n      \&quot;PersentaseInsentifDicadangkan\&quot;: 2.0,\n      \&quot;NominalInsentifDicadangkan\&quot;: 3.0\n    }\n  ],\n  \&quot;RefundRateyangDibagikan\&quot;: [\n    {\n      \&quot;IdPenerima\&quot;: 1,\n      \&quot;AccountNo\&quot;: \&quot;sample string 2\&quot;,\n      \&quot;PersentaseInsentifDibagikan\&quot;: 3.0,\n      \&quot;NominalInsentifDibagikan\&quot;: 4.0\n    },\n    {\n      \&quot;IdPenerima\&quot;: 1,\n      \&quot;AccountNo\&quot;: \&quot;sample string 2\&quot;,\n      \&quot;PersentaseInsentifDibagikan\&quot;: 3.0,\n      \&quot;NominalInsentifDibagikan\&quot;: 4.0\n    }\n  ],\n  \&quot;RefundRateyangDicadangkan\&quot;: [\n    {\n      \&quot;JenisCadangan\&quot;: \&quot;sample string 1\&quot;,\n      \&quot;PersentaseInsentifDicadangkan\&quot;: 2.0,\n      \&quot;NominalInsentifDicadangkan\&quot;: 3.0\n    },\n    {\n      \&quot;JenisCadangan\&quot;: \&quot;sample string 1\&quot;,\n      \&quot;PersentaseInsentifDicadangkan\&quot;: 2.0,\n      \&quot;NominalInsentifDicadangkan\&quot;: 3.0\n    }\n  ],\n  \&quot;InaturaProvisiyangDibagikan\&quot;: [\n    {\n      \&quot;IdPenerima\&quot;: 1,\n      \&quot;AccountNo\&quot;: \&quot;sample string 2\&quot;,\n      \&quot;PersentaseInsentifDibagikan\&quot;: 3.0,\n      \&quot;NominalInsentifDibagikan\&quot;: 4.0\n    },\n    {\n      \&quot;IdPenerima\&quot;: 1,\n      \&quot;AccountNo\&quot;: \&quot;sample string 2\&quot;,\n      \&quot;PersentaseInsentifDibagikan\&quot;: 3.0,\n      \&quot;NominalInsentifDibagikan\&quot;: 4.0\n    }\n  ],\n  \&quot;InaturaProvisiyangDicadangkan\&quot;: [\n    {\n      \&quot;JenisCadangan\&quot;: \&quot;sample string 1\&quot;,\n      \&quot;PersentaseInsentifDicadangkan\&quot;: 2.0,\n      \&quot;NominalInsentifDicadangkan\&quot;: 3.0\n    },\n    {\n      \&quot;JenisCadangan\&quot;: \&quot;sample string 1\&quot;,\n      \&quot;PersentaseInsentifDicadangkan\&quot;: 2.0,\n      \&quot;NominalInsentifDicadangkan\&quot;: 3.0\n    }\n  ],\n  \&quot;InaturaProgramyangDibagikan\&quot;: [\n    {\n      \&quot;IdPenerima\&quot;: 1,\n      \&quot;AccountNo\&quot;: \&quot;sample string 2\&quot;,\n      \&quot;PersentaseInsentifDibagikan\&quot;: 3.0,\n      \&quot;NominalInsentifDibagikan\&quot;: 4.0\n    },\n    {\n      \&quot;IdPenerima\&quot;: 1,\n      \&quot;AccountNo\&quot;: \&quot;sample string 2\&quot;,\n      \&quot;PersentaseInsentifDibagikan\&quot;: 3.0,\n      \&quot;NominalInsentifDibagikan\&quot;: 4.0\n    }\n  ],\n  \&quot;InaturaProgramyangDicadangkan\&quot;: [\n    {\n      \&quot;JenisCadangan\&quot;: \&quot;sample string 1\&quot;,\n      \&quot;PersentaseInsentifDicadangkan\&quot;: 2.0,\n      \&quot;NominalInsentifDicadangkan\&quot;: 3.0\n    },\n    {\n      \&quot;JenisCadangan\&quot;: \&quot;sample string 1\&quot;,\n      \&quot;PersentaseInsentifDicadangkan\&quot;: 2.0,\n      \&quot;NominalInsentifDicadangkan\&quot;: 3.0\n    }\n  ],\n  \&quot;KomisiAdminyangDibagikan\&quot;: [\n    {\n      \&quot;IdPenerima\&quot;: 1,\n      \&quot;AccountNo\&quot;: \&quot;sample string 2\&quot;,\n      \&quot;PersentaseInsentifDibagikan\&quot;: 3.0,\n      \&quot;NominalInsentifDibagikan\&quot;: 4.0\n    },\n    {\n      \&quot;IdPenerima\&quot;: 1,\n      \&quot;AccountNo\&quot;: \&quot;sample string 2\&quot;,\n      \&quot;PersentaseInsentifDibagikan\&quot;: 3.0,\n      \&quot;NominalInsentifDibagikan\&quot;: 4.0\n    }\n  ],\n  \&quot;KomisiAdminyangDicadangkan\&quot;: [\n    {\n      \&quot;JenisCadangan\&quot;: \&quot;sample string 1\&quot;,\n      \&quot;PersentaseInsentifDicadangkan\&quot;: 2.0,\n      \&quot;NominalInsentifDicadangkan\&quot;: 3.0\n    },\n    {\n      \&quot;JenisCadangan\&quot;: \&quot;sample string 1\&quot;,\n      \&quot;PersentaseInsentifDicadangkan\&quot;: 2.0,\n      \&quot;NominalInsentifDicadangkan\&quot;: 3.0\n    }\n  ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>47827839-534a-44d8-a971-86d59090aa31</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic QkFTSUNfQVVUSDpNb2JpbGUxMjMr</value>
      <webElementGuid>7c06dab6-0410-4790-9b6f-558c7ece0e23</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://192.168.1.254:8089/api/PermohonanPembayaranKomisiDealer/Proceed</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
