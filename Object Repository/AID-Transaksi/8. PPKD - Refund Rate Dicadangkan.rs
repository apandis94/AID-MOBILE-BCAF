<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>8. PPKD - Refund Rate Dicadangkan</name>
   <tag></tag>
   <elementGuidId>73280cca-1601-40b8-ad3e-2793344bcc23</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;NoKontrak\&quot;: \&quot;${accno}\&quot;,\n  \&quot;JenisCadangan\&quot;: \&quot;${jenis_refundrate_cad}\&quot;,\n  \&quot;Persentase\&quot;: ${refundrate_percent},\n  \&quot;SukuBunga\&quot;: ${sukubunga},\n  \&quot;NominalKomisi\&quot;: ${amnt_komisi}\n}\n&quot;,
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
      <webElementGuid>23eed3ac-367f-4b33-9221-89e44ce5014c</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic QkFTSUNfQVVUSDpNb2JpbGUxMjMr</value>
      <webElementGuid>425e9d2b-e173-4e19-945c-0b92a611f554</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://192.168.1.254:8089/api/PermohonanPembayaranKomisiDealer/RefundRate/Dicadangkan/NominalKomisi</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'9530708518-001'</defaultValue>
      <description></description>
      <id>b1357167-4e14-40c8-96a2-0241fcd47ad8</id>
      <masked>false</masked>
      <name>accno</name>
   </variables>
   <variables>
      <defaultValue>'Cadangan BM'</defaultValue>
      <description></description>
      <id>26a44fd6-3d43-4981-a292-3a3005c8a399</id>
      <masked>false</masked>
      <name>jenis_refundrate_cad</name>
   </variables>
   <variables>
      <defaultValue>4.0</defaultValue>
      <description></description>
      <id>70b60a2f-9919-40bd-838c-4927839d480a</id>
      <masked>false</masked>
      <name>refundrate_percent</name>
   </variables>
   <variables>
      <defaultValue>11</defaultValue>
      <description></description>
      <id>06a40472-2a74-481b-b64b-eb3b941a591b</id>
      <masked>false</masked>
      <name>sukubunga</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>0a9f0e69-b184-4645-9687-06896f9ce78d</id>
      <masked>false</masked>
      <name>amnt_komisi</name>
   </variables>
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
